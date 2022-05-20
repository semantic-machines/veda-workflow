import 'adoptedStyleSheets';
import $ from 'jquery';
import IndividualModel from '/js/common/individual_model.js';
import notify from '/js/browser/notify.js';

export const post = async function (individual, template, container, mode, extra) {
  template = $(template);
  container = $(container);

  const styles = [
    (await import('bpmn-js/assets/diagram-js.css')).default,
    (await import('bpmn-js/assets/bpmn-js.css')).default,
    (await import('bpmn-js/assets/bpmn-font/css/bpmn-embedded.css')).default,
    (await import('bpmn-js-properties-panel/assets/properties-panel.css')).default,
  ];
  document.adoptedStyleSheets = [...document.adoptedStyleSheets, ...styles];

  const BpmnModeler = (await import('bpmn-js/bpmn-modeler.production.min.js')).default;
  const BpmnViewer = (await import('bpmn-js/bpmn-navigated-viewer.production.min.js')).default;
  const {BpmnPropertiesPanelModule, BpmnPropertiesProviderModule, CamundaPlatformPropertiesProviderModule} = (await import('bpmn-js-properties-panel/bpmn-js-properties-panel.umd.js')).default;
  const BpmnModdle = (await import('bpmn-moddle')).default;
  const CamundaBpmnModdle = (await import('camunda-bpmn-moddle')).default;

  const random = () => Date.now() + Math.floor(performance.now());
  const startEventId = random();
  const processId = random();
  const emptyDefinition = `<?xml version="1.0" encoding="UTF-8"?><bpmn2:definitions xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:bpmn2="http://www.omg.org/spec/BPMN/20100524/MODEL" xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" xmlns:di="http://www.omg.org/spec/DD/20100524/DI" xsi:schemaLocation="http://www.omg.org/spec/BPMN/20100524/MODEL BPMN20.xsd" id="Definitions_${random()}" targetNamespace="http://bpmn.io/schema/bpmn"><bpmn2:process id="Process_${processId}" isExecutable="false"><bpmn2:startEvent id="StartEvent_${startEventId}"/></bpmn2:process><bpmndi:BPMNDiagram id="BPMNDiagram_${random()}"><bpmndi:BPMNPlane id="BPMNPlane_${random()}" bpmnElement="Process_1"><bpmndi:BPMNShape id="_BPMNShape_StartEvent_${random()}" bpmnElement="StartEvent_${startEventId}"><dc:Bounds height="36.0" width="36.0" x="412.0" y="240.0"/></bpmndi:BPMNShape></bpmndi:BPMNPlane></bpmndi:BPMNDiagram></bpmn2:definitions>`;
  const definition = individual.hasValue('bpmn:processDefinition') ? individual['bpmn:processDefinition'][0].toString() : emptyDefinition;

  const viewer = new BpmnViewer({
    container: '#viewer-diagram',
  });

  const modeler = new BpmnModeler({
    container: '#modeler-diagram',
    propertiesPanel: {
      parent: '#modeler-properties-panel',
    },
    additionalModules: [
      BpmnPropertiesPanelModule,
      BpmnPropertiesProviderModule,
      //CamundaPlatformPropertiesProviderModule,
    ],
    moddleExtensions: {
      //camunda: CamundaBpmnModdle,
      bpmn2: BpmnModdle,
    },
  });

  try {
    await viewer.importXML(definition);
    viewer.get('canvas').zoom(1);
  } catch (error) {
    notify('danger', error);
  }

  try {
    await modeler.importXML(definition);
    modeler.get('canvas').zoom(1);
  } catch (error) {
    notify('danger', error);
  }

  individual.on('bpmn:processDefinition', updateView);
  async function updateView ([value]) {
    try {
      await viewer.importXML(value.toString());
      viewer.get('canvas').zoom(1);
    } catch (error) {
      notify('danger', error);
    }
  }

  $('#save', template).on('click', saveProcess);
  async function saveProcess () {
    try {
      const updatedDefinition = await modeler.saveXML();
      individual['bpmn:processDefinition'] = updatedDefinition.xml;
      template[0].dispatchEvent(new Event('save'));
    } catch (error) {
      notify('danger', {name: error.name, message: error.stack});
    }
  }

  $('#download-xml', template).on('click', openXML);
  async function openXML () {
    try {
      const diagram = await modeler.saveXML();
      const blob = new Blob([diagram.xml], {type: 'text/xml'});
      const url = URL.createObjectURL(blob);
      window.open(url);
      URL.revokeObjectURL(url);
    } catch (error) {
      console.log(error);
      const errorMsg = await new IndividualModel('v-s:ErrorBundle').load();
      notify('danger', {name: errorMsg.toString()});
    }
  }

  $('#upload-xml', template).on('click', () => $('#upload-xml-input', template).click());
  $('#upload-xml-input', template).on('change', (e) => {
    const xmlFile = e.target.files[0];
    const reader = new FileReader();
    reader.onload = async function () {
      try {
        const xml = reader.result;
        await modeler.importXML(xml);
        modeler.get('canvas').zoom(1);
      } catch (error) {
        notify('danger', error);
      }
    };
    reader.readAsText(xmlFile);
  });

  $(document).on('keydown', shortcuts);

  function shortcuts (e) {
    // Ctrl + z
    if (e.keyCode === 90 && e.ctrlKey) {
      modeler.get('commandStack').undo();
    }
    // Ctrl + y
    if (e.keyCode === 89 && e.ctrlKey) {
      modeler.get('commandStack').redo();
    }
    // Ctrl + s
    if (e.keyCode === 83 && e.ctrlKey && e.altKey) {
      e.preventDefault();
      saveProcess();
    }
    // Ctrl + e
    if (e.keyCode === 69 && e.ctrlKey && e.altKey) {
      e.preventDefault();
      template[0].dispatchEvent(new Event('edit'));
    }
  }
  template.one('remove', function () {
    $(document).off('keydown', shortcuts);
  });
};

export const html = `
  <div class="container-fluid sheet">
    <style scoped>
      .content {
        display: flex;
      }
      #modeler-diagram, #viewer-diagram {
        width: 100%;
        height: ${window.innerHeight - 250}px;
      }
      #modeler-properties-panel, #viewer-properties-panel {
        width: 400px;
        border-left: 1px solid #ccc;
        overflow: hidden;
      }
    </style>
    <h4 about="@" property="rdfs:label" class="text-center view -edit -search"></h4>
    <veda-control data-type="string" property="rdfs:label" class="-view edit search"></veda-control>
    <div class="content view -edit">
      <div id="viewer-diagram"></div>
      <div id="viewer-properties-panel">
        <div class="container-fluid">
          <em about="rdf:type" property="rdfs:label"></em>
          <span rel="rdf:type" data-template="v-ui:LabelTemplate"></span>
          <em about="rdfs:label" property="rdfs:label"></em>
          <span property="rdfs:label"></span>
          <em about="v-s:creator" property="rdfs:label"></em>
          <span rel="v-s:creator" data-template="v-ui:LabelTemplate"></span>
          <em about="v-s:created" property="rdfs:label"></em>
          <span property="v-s:created"></span>
          <em about="v-s:edited" property="rdfs:label"></em>
          <span property="v-s:edited"></span>
        </div>
      </div>
    </div>
    <div class="content -view edit">
      <div id="modeler-diagram"></div>
      <div id="modeler-properties-panel"></div>
    </div>
    <div class="actions">
      <button class="btn btn-success -view edit -search" id="save" about="v-s:Save" property="rdfs:label"></button>
      <span about="@" data-template="v-ui:StandardButtonsTemplate" data-embedded="true" data-buttons="edit cancel"></span>
      <button class="btn btn-default -view edit -search" id="upload-xml">Upload XML</button>
      <input type="file" id="upload-xml-input" class="-view -edit -search"></input>
      <button class="btn btn-default view edit -search" id="download-xml">View XML</button>
    </div>
  </div>
`;
