package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.delegate.DelegateExecution;
import org.camunda.bpm.engine.delegate.DelegateTask;
import org.camunda.bpm.engine.delegate.TaskListener;
import org.camunda.bpm.engine.impl.persistence.entity.ExecutionEntity;

import java.util.logging.Logger;

/**
 * Task listener to be executed when a user task instance event is fired
 */
public class VedaUserTaskListener implements TaskListener {
  
  private static VedaQueueWriter queueWriter ;
  private static VedaUserTaskListener instance = null;
  private final Logger LOGGER = Logger.getLogger(this.getClass().getName());
  public static long callCounter = 0;

  protected VedaUserTaskListener() {
    queueWriter = VedaQueueWriter.getInstance();
  }
  
  public static VedaUserTaskListener getInstance() {
    if (instance == null) {
      instance = new VedaUserTaskListener();
    }
    return instance;
  }

  /**
   * Put a message in the queue with the following format:
   * UserTaskEvent:{event},{taskId},{processInstanceId},{processDefinitionKey},{elementType},{elementId}
   */
  public void notify(DelegateTask delegateTask) {
    callCounter++;
    String event = delegateTask.getEventName();
    String taskId = delegateTask.getId();
    String processDefinitionKey = getProcessDefinitionKey(delegateTask.getProcessDefinitionId());
    String elementType = delegateTask.getBpmnModelElementInstance().getElementType().getTypeName();
    String elementId = delegateTask.getTaskDefinitionKey();
    String processInstanceId = delegateTask.getProcessInstanceId();
    DelegateExecution execution = delegateTask.getExecution();
    String businessKey = execution.getBusinessKey();

    ExecutionEntity executionEntity = (ExecutionEntity) execution;
    ExecutionEntity processInstance = executionEntity.getProcessInstance();
    ExecutionEntity superExecution = processInstance.getSuperExecution();
    String superProcessInstanceId = null;
    if(superExecution != null) {
      superProcessInstanceId = superExecution.getProcessInstanceId();
    }
    
    String msg = "UserTaskEvent:" + String.join(",", event, taskId, processInstanceId, superProcessInstanceId, businessKey, processDefinitionKey, elementType, elementId);
    queueWriter.queue.push(msg);
    LOGGER.info("queue: " + msg);
  }

  private String getProcessDefinitionKey(String processDefinitionId) {
    String idPattern = "^(.+?):.*$"; 
    return processDefinitionId.replaceAll(idPattern, "$1");
  }

}