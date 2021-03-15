package com.semanticmachines.veda.bpm;

import java.util.logging.Logger;

import org.camunda.bpm.engine.delegate.DelegateExecution;
import org.camunda.bpm.engine.delegate.ExecutionListener;

public class VedaExecutionListener implements ExecutionListener {

  private final Logger LOGGER = Logger.getLogger(this.getClass().getName());
  public static long callCounter = 0;
  
  private static VedaExecutionListener instance = null;
  
  public static VedaExecutionListener getInstance() {
    if (instance == null) {
        instance = new VedaExecutionListener();
    }
    return instance;
  }

  protected VedaExecutionListener() {
  }

  public void notify(DelegateExecution execution) throws Exception {
    callCounter++;
    String event = execution.getEventName();
    LOGGER.info("[" + callCounter + "] Task event: '" + event + "', execution: " + execution);
  }

}
