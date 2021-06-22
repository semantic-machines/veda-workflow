package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.impl.bpmn.parser.BpmnParseListener;
import org.camunda.bpm.engine.impl.cfg.AbstractProcessEnginePlugin;
import org.camunda.bpm.engine.impl.cfg.ProcessEngineConfigurationImpl;
import org.camunda.bpm.engine.impl.cfg.ProcessEnginePlugin;
import org.camunda.bpm.engine.impl.persistence.deploy.Deployer;

import java.util.ArrayList;
import java.util.List;

/**
 * <p>{@link ProcessEnginePlugin} enabling the assignee informing parse listener.</p>
 *
 */
public class VedaCamundaPlugin extends AbstractProcessEnginePlugin {

  @Override
  public void preInit(ProcessEngineConfigurationImpl processEngineConfiguration) {
    
    // Add custom parse listener
    List<BpmnParseListener> preParseListeners = processEngineConfiguration.getCustomPreBPMNParseListeners();
    if(preParseListeners == null) {
      preParseListeners = new ArrayList<BpmnParseListener>();
      processEngineConfiguration.setCustomPreBPMNParseListeners(preParseListeners);
    }
    preParseListeners.add(new VedaParseListener());

    // Add custom deployer
    List<Deployer> postDeployListeners = processEngineConfiguration.getCustomPostDeployers();
    if(postDeployListeners == null) {
      postDeployListeners = new ArrayList<Deployer>();
      processEngineConfiguration.setCustomPostDeployers(postDeployListeners);
    }
    postDeployListeners.add(new VedaDeployer());
  }

}