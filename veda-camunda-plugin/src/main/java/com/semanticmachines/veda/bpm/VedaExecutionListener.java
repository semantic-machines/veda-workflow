package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.delegate.DelegateExecution;
import org.camunda.bpm.engine.delegate.ExecutionListener;
import org.camunda.bpm.engine.impl.persistence.entity.ExecutionEntity;
import org.camunda.bpm.model.xml.instance.ModelElementInstance;
import org.camunda.bpm.model.xml.type.ModelElementType;

import java.util.logging.Logger;

/**
 * Execution listener to be executed when an execution event is fired
 */
public class VedaExecutionListener implements ExecutionListener {
  
  private static VedaQueueWriter queueWriter ;
  private static VedaExecutionListener instance = null;
  private final Logger LOGGER = Logger.getLogger(this.getClass().getName());
  public static long callCounter = 0;

  protected VedaExecutionListener() {
    queueWriter = VedaQueueWriter.getInstance();
  }
  
  public static VedaExecutionListener getInstance() {
    if (instance == null) {
        instance = new VedaExecutionListener();
    }
    return instance;
  }
  
  /**
   * Put a message in the queue with the following format:
   * ExecutionEvent:{event},{executionId},{processInstanceId},{processDefinitionKey},{elementType},{elementId}
   */
  public void notify(DelegateExecution execution) throws Exception {
    callCounter++;
    String event = execution.getEventName();
    String executionId = execution.getId();
    String processDefinitionKey = getProcessDefinitionKey(execution.getProcessDefinitionId());
    
    String elementType = null;
    ModelElementInstance modelInstance = (ModelElementInstance) execution.getBpmnModelElementInstance();
    if (modelInstance != null) {
      ModelElementType modelElementType = modelInstance.getElementType();
      if (modelElementType != null) {
        elementType = modelElementType.getTypeName();
      }
    }

    String elementId;
    if (event == "take") {
      elementId = execution.getCurrentTransitionId();
    } else {
      elementId = execution.getCurrentActivityId();
    }
    String processInstanceId = execution.getProcessInstanceId();
    
    ExecutionEntity executionEntity = (ExecutionEntity) execution;
    ExecutionEntity processInstance = executionEntity.getProcessInstance();
    ExecutionEntity superExecution = processInstance.getSuperExecution();
    String superProcessInstanceId = null;
    if(superExecution != null) {
      superProcessInstanceId = superExecution.getProcessInstanceId();
    }
    
    String businessKey = execution.getBusinessKey();
    String msg = "ExecutionEvent:" + String.join(",", event, executionId, processInstanceId, superProcessInstanceId, businessKey, processDefinitionKey, elementType, elementId);
    queueWriter.queue.push(msg);
    LOGGER.info("queue: " + msg);
  }

  private String getProcessDefinitionKey(String processDefinitionId) {
    String idPattern = "^(.+?):.*$"; 
    return processDefinitionId.replaceAll(idPattern, "$1");
  }
}