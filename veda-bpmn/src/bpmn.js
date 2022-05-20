import {Engine} from 'bpmn-engine';
import {EventEmitter} from 'events';
import './polyfill.js';
import log from 'loglevel';
import IndividualModel from 'veda/js/common/individual_model.js';
import camundaExtensions from './camundaExtensions.js';
import fs from 'fs';
const camundaBpmnModdle = JSON.parse(fs.readFileSync('./node_modules/camunda-bpmn-moddle/resources/camunda.json'));

class Emitter extends EventEmitter {
  emit (type, api, execution) {
    // log.debug(new Date().toISOString(), type, 'emitted', 'variables:', JSON.stringify(api.environment.variables));
    log.debug(new Date().toISOString(), type, 'emitted', api.type, api.name);
    super.emit(type, api, execution);
  }
}

export default class Bpmn {
  static async startProcess (startForm) {
    log.info(new Date().toISOString(), `Start process, form = ${startForm.id}`);
    log.debug(new Date().toISOString(), `Start process, form = ${JSON.stringify(startForm.properties)}`);

    const processDefinition = await getProcessDefinition(startForm);

    const engine = new Engine({
      source: processDefinition,
      moddleOptions: {
        camunda: camundaBpmnModdle,
      },
      extensions: {
        camunda: camundaExtensions,
      },
    });

    const processInstance = new IndividualModel();
    processInstance['rdf:type'] = 'bpmn:ProcessInstance';
    processInstance['bpmn:instanceOf'] = process;
    processInstance['bpmn:hasStartForm'] = startForm;

    const listener = new Emitter();
    listener.on('process.start', async () => {
      try {
        startForm.set('bpmn:hasStatus', 'bpmn:Started');
        startForm.set('bpmn:hasProcessInstance', processInstance);
        await startForm.save();
      } catch (error) {
        log.error(new Date().toISOString(), `Failed to update start form = ${startForm.id}`, error.stack);
        throw error;
      }
    });

    const saver = async (api, execution) => await saveState(execution, processInstance);
    listener.on('process.start', saver);
    listener.on('activity.wait', saver);
    listener.on('process.stop', saver);
    listener.on('activity.start', (...args) => {
      args;
    });

    const services = {
      IndividualModel,
      fetch,
      log,
    };

    engine.execute({
      listener,
      variables: {
        startForm: startForm.properties,
      },
      services,
    }, (error, execution) => {
      if (error) {
        log.error(new Date().toISOString(), `Failed to execute process, form = ${startForm}`, error.stack);
        throw error;
      }
      log.info(new Date().toISOString(), `Execution completed successfully, process instance =  ${processInstance.id}, form = ${startForm.id}, variables = `, execution.environment.variables);
    });
  }

  static async handleTaskForm (taskForm) {}

  static async handleSuspendedProcess (process) {}

  static async handleCompletedProcess (process) {}
}

async function saveState (execution, processInstance) {
  log.info(new Date().toISOString(), `Save process state, process instance = ${processInstance.id}`);
  await execution.stop();
  const state = JSON.stringify(await execution.getState());
  try {
    await processInstance.reset();
    processInstance.set('bpmn:processState', state);
    await processInstance.save(false);
  } catch (error) {
    log.error(new Date().toISOString(), `Failed to save process state, process instance = ${processInstance.id}`, error.stack);
    throw error;
  }
}

async function getProcessDefinition (startForm) {
  const processId = startForm.hasValue('bpmn:startProcess') && startForm.get('bpmn:startProcess')[0].id;
  if (!processId) {
    log.error(new Date().toISOString(), `Process is undefined, form = ${startForm.id}`);
    return;
  }

  let process;
  try {
    process = await new IndividualModel(processId).reset();
    log.info(new Date().toISOString(), `Process = ${process.id}`);
    log.debug(new Date().toISOString(), `Process = ${JSON.stringify(process.properties)}`);
  } catch (error) {
    log.error(new Date().toISOString(), `Failed to load process individual, process = ${process.id}, form = ${startForm.id}`, error.stack);
    throw error;
  }

  const processDefinition = process.get('bpmn:processDefinition')[0].toString();
  if (!processDefinition) {
    log.error(new Date().toISOString(), `Process individual lacks process definition, process = ${process.id}`);
    return;
  }
  return processDefinition;
}
