package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.delegate.DelegateTask;
import org.camunda.bpm.engine.delegate.TaskListener;
import java.util.logging.Logger;

/**
 * Task listener to be executed when a user task instance event is fired
 */
public class VedaUserTaskListener implements TaskListener {
  
  private static VedaQueueWriter queueWriter ;
  private static VedaUserTaskListener instance = null;
  private final Logger LOGGER = Logger.getLogger(this.getClass().getName());
  public static long callCounter = 0;

  protected VedaUserTaskListener() {}
  
  public static VedaUserTaskListener getInstance() {
    if (instance == null) {
      instance = new VedaUserTaskListener();
    }
    queueWriter = VedaQueueWriter.getInstance(); 
    return instance;
  }

  // Possible events = create, assignment, complete, update, delete, timeout
  public void notify(DelegateTask delegateTask) {
    callCounter++;
    String event = delegateTask.getEventName();
    String taskId = delegateTask.getId();
    String processDefinitionKey = getProcessDefinitionKey(delegateTask.getProcessDefinitionId());
    String elementType = delegateTask.getBpmnModelElementInstance().getElementType().getTypeName();
    String elementId = delegateTask.getTaskDefinitionKey();
    String msg = "UserTaskEvent:" + String.join(",", event, taskId, processDefinitionKey, elementType, elementId);
    queueWriter.queue.push(msg);
    LOGGER.info("queue: " + msg);
  }

  private String getProcessDefinitionKey(String processDefinitionId) {
    String idPattern = "^(.+?):.+?:.+?$"; 
    return processDefinitionId.replaceAll(idPattern, "$1");
  }

}