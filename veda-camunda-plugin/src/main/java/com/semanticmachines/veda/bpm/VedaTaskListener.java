package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.delegate.DelegateTask;
import org.camunda.bpm.engine.delegate.TaskListener;
import java.util.logging.Logger;

/**
 * Task listener to be executed when a user task instance event is fired
 */
public class VedaTaskListener implements TaskListener {
  
  private static VedaQueueWriter queueWriter ;
  private static VedaTaskListener instance = null;
  private final Logger LOGGER = Logger.getLogger(this.getClass().getName());
  public static long callCounter = 0;

  protected VedaTaskListener() {}
  
  public static VedaTaskListener getInstance() {
    if (instance == null) {
      instance = new VedaTaskListener();
    }
    queueWriter = VedaQueueWriter.getInstance(); 
    return instance;
  }

  // Possible events = create, assignment, complete, update, delete, timeout
  public void notify(DelegateTask delegateTask) {
    callCounter++;
    String event = delegateTask.getEventName();
    LOGGER.info("[" + callCounter + "] Task event: '" + event + "', task: " + delegateTask);

    queueWriter.queue.push(event + "," + delegateTask);
  }
  
}