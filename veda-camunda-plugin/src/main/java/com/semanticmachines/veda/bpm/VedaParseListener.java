package com.semanticmachines.veda.bpm;

import java.util.List;
import org.camunda.bpm.engine.impl.bpmn.behavior.UserTaskActivityBehavior;
import org.camunda.bpm.engine.impl.bpmn.parser.AbstractBpmnParseListener;
import org.camunda.bpm.engine.impl.persistence.entity.ProcessDefinitionEntity;
import org.camunda.bpm.engine.impl.pvm.delegate.ActivityBehavior;
import org.camunda.bpm.engine.impl.pvm.process.ActivityImpl;
import org.camunda.bpm.engine.impl.pvm.process.ScopeImpl;
import org.camunda.bpm.engine.impl.pvm.process.TransitionImpl;
import org.camunda.bpm.engine.impl.task.TaskDefinition;
import org.camunda.bpm.engine.impl.util.xml.Element;
import org.camunda.bpm.engine.impl.variable.VariableDeclaration;

/**
 * BPMN Parse Listener to add task listener on user task
 *
 */
public class VedaParseListener extends AbstractBpmnParseListener {

  VedaUserTaskListener taskListener = VedaUserTaskListener.getInstance();
  VedaExecutionListener executionListener = VedaExecutionListener.getInstance();
  
  @Override
  public void parseUserTask(Element userTaskElement, ScopeImpl scope, ActivityImpl activity) {
    ActivityBehavior activityBehavior = activity.getActivityBehavior();
    if(activityBehavior instanceof UserTaskActivityBehavior) {
      UserTaskActivityBehavior userTaskActivityBehavior = (UserTaskActivityBehavior) activityBehavior;
      TaskDefinition task = userTaskActivityBehavior.getTaskDefinition();
      task.addTaskListener("create", taskListener);
      task.addTaskListener("assignment", taskListener);
      task.addTaskListener("update", taskListener);
      task.addTaskListener("delete", taskListener);
      task.addTaskListener("timeout", taskListener);
      task.addTaskListener("complete", taskListener);
    }
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseProcess(Element processElement, ProcessDefinitionEntity processDefinition) {
    processDefinition.addListener("start", executionListener);
    processDefinition.addListener("end", executionListener);
  }

  @Override
  public void parseStartEvent(Element startEventElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseExclusiveGateway(Element exclusiveGwElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }
  
  @Override
  public void parseInclusiveGateway(Element inclusiveGwElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseParallelGateway(Element parallelGwElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseScriptTask(Element scriptTaskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseServiceTask(Element serviceTaskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBusinessRuleTask(Element businessRuleTaskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseTask(Element taskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseManualTask(Element manualTaskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseEndEvent(Element endEventElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundaryTimerEventDefinition(Element timerEventDefinition, boolean interrupting, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundaryErrorEventDefinition(Element errorEventDefinition, boolean interrupting, ActivityImpl activity, ActivityImpl nestedErrorEventActivity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseSubProcess(Element subProcessElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseCallActivity(Element callActivityElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseProperty(Element propertyElement, VariableDeclaration variableDeclaration, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseSequenceFlow(Element sequenceFlowElement, ScopeImpl scopeElement, TransitionImpl transition) {
    transition.addListener("take", executionListener);
  }

  @Override
  public void parseSendTask(Element sendTaskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseMultiInstanceLoopCharacteristics(Element activityElement, Element multiInstanceLoopCharacteristicsElement, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseIntermediateTimerEventDefinition(Element timerEventDefinition, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseRootElement(Element rootElement, List<ProcessDefinitionEntity> processDefinitions) {
  }

  @Override
  public void parseReceiveTask(Element receiveTaskElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseIntermediateSignalCatchEventDefinition(Element signalEventDefinition, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundarySignalEventDefinition(Element signalEventDefinition, boolean interrupting, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseEventBasedGateway(Element eventBasedGwElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseTransaction(Element transactionElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseCompensateEventDefinition(Element compensateEventDefinition, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseIntermediateThrowEvent(Element intermediateEventElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseIntermediateCatchEvent(Element intermediateEventElement, ScopeImpl scope, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundaryEvent(Element boundaryEventElement, ScopeImpl scopeElement, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseIntermediateMessageCatchEventDefinition(Element messageEventDefinition, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundaryMessageEventDefinition(Element element, boolean interrupting, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundaryEscalationEventDefinition(Element escalationEventDefinition, boolean interrupting, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseBoundaryConditionalEventDefinition(Element element, boolean interrupting, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseIntermediateConditionalEventDefinition(Element conditionalEventDefinition, ActivityImpl activity) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }

  @Override
  public void parseConditionalStartEventForEventSubprocess(Element element, ActivityImpl activity, boolean interrupting) {
    activity.addListener("start", executionListener);
    activity.addListener("end", executionListener);
  }  
  
}
