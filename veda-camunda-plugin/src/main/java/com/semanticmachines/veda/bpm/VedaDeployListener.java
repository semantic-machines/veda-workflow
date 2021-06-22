package com.semanticmachines.veda.bpm;

import java.util.List;
import java.util.logging.Logger;

import org.camunda.bpm.engine.impl.persistence.deploy.Deployer;
import org.camunda.bpm.engine.impl.persistence.entity.DeploymentEntity;
import org.camunda.bpm.engine.repository.ProcessDefinition;

public class VedaDeployListener implements Deployer {

  private static VedaQueueWriter queueWriter = VedaQueueWriter.getInstance();
  private final Logger LOGGER = Logger.getLogger(this.getClass().getName());

  @Override
  public void deploy(DeploymentEntity deployment) {
    // TODO Auto-generated method stub
    List<ProcessDefinition> processDefinitions = deployment.getDeployedProcessDefinitions();
    processDefinitions.forEach((processDefinition) -> {
      String msg = "ProcessDefinitionEvent:" + String.join(",", "deploy", processDefinition.getKey(), processDefinition.getId());
      queueWriter.queue.push(msg);
      LOGGER.info("queue: " + msg);
    });
  }

}