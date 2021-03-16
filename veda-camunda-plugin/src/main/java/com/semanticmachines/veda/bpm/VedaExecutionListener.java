package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.delegate.DelegateExecution;
import org.camunda.bpm.engine.delegate.ExecutionListener;

import java.util.logging.Logger;

/**
 * Execution listener to be executed when a process instance event is fired
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
  
  /*
   * Put a message in the queue with the following format:
   * ExecutionEvent:{event},{executionId},{processInstanceId},{processDefinitionKey},{elementType},{elementId}
   */
  public void notify(DelegateExecution execution) throws Exception {
    callCounter++;
    String event = execution.getEventName();
    String executionId = execution.getId();
    String processDefinitionKey = getProcessDefinitionKey(execution.getProcessDefinitionId());
    String elementType = execution.getBpmnModelElementInstance().getElementType().getTypeName();
    String elementId;
    if (event == "take") {
      elementId = execution.getCurrentTransitionId();
    } else {
      elementId = execution.getCurrentActivityId();
    }
    String processInstanceId = execution.getProcessInstanceId();
    String msg = "ExecutionEvent:" + String.join(",", event, executionId, processInstanceId, processDefinitionKey, elementType, elementId);
    queueWriter.queue.push(msg);
    LOGGER.info("queue: " + msg);
  }

  private String getProcessDefinitionKey(String processDefinitionId) {
    String idPattern = "^(.+?):.+?:.+?$"; 
    return processDefinitionId.replaceAll(idPattern, "$1");
  }
}