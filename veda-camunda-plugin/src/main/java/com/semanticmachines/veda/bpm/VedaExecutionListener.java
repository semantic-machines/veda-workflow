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

  protected VedaExecutionListener() {}
  
  public static VedaExecutionListener getInstance() {
    if (instance == null) {
        instance = new VedaExecutionListener();
    }
    queueWriter = VedaQueueWriter.getInstance();
    return instance;
  }
  
  //Possible events = start, end
  public void notify(DelegateExecution execution) throws Exception {
    callCounter++;
    String event = execution.getEventName();
    LOGGER.info("[" + callCounter + "] Task event: '" + event + "', execution: " + execution);
    
    queueWriter.queue.push(event + "," + execution);
  }

}