import BpmnModeler from 'bpmn-js';
import $ from 'jquery';
import IndividualModel from '/js/common/individual_model.js';

export const post = function (individual, template, container, mode, extra) {
  const viewer = new BpmnModeler({
    container: "#veda-bpmn"
  });
  template.one("remove", function () {
    viewer.destroy();
  });

  const overlays = viewer.get("overlays");

  const process = individual["bpmn:instanceOf"][0];
  process.load()
    .then(function (process) {
       const definition = process["bpmn:processDefinition"][0].toString();
       viewer.importXML(definition);
    });

  individual.on("bpmn:hasToken", presentTokens);
  template.one("remove", function () {
    individual.off("bpmn:hasToken", presentTokens);
  });
  presentTokens();

  function presentTokens() {
    const tokens = individual["bpmn:hasToken"];

    overlays.remove({type: "token"});

    const tokensPromises = tokens.map(function (token) {
      return token.load();
    });
    Promise.all(tokensPromises)
      .then(function (tokens) {
        tokens.forEach(presentToken);
      });
  }

  function presentToken (token, index) {
    const elementId = token["bpmn:elementId"][0].toString();
    const elementTokens = overlays.get({ element: elementId, type: 'token' });
    const i = elementTokens.length;

    const token_chip = $('<div class="token-chip" resource="' + token.id + '">' + (index + 1) + '</div>');

    overlays.add(elementId, "token", {
      position: {
        top: -25,
        left: 25 * i
      },
      html: token_chip
    });

    const cntr = $("<div></div>");
    const tmpl = "bpmn:TokenPopoverTemplate";
    token.present(cntr, tmpl).then(function () {
      token_chip.popover({
        trigger: "click",
        placement: "top",
        html: true,
        container: this,
        content: cntr
      });
    });

    token.one("propertyModified", presentTokens);
    template.one("remove", function () {
      token.off("propertyModified", presentTokens);
    });
  }
};

export const html = `
  <div class="container-fluid sheet">
    <style scoped>
      .token-chip {
        height: 20px;
        min-width: 20px;
        display: block;
        text-align: center;
        vertical-align: middle;
        border-radius: 50%;
        background: orangered;
        color: white;
        font-weight: bold;
      }
      .token-chip:hover {
        color: white;
        text-decoration:none;
        cursor: pointer;
      }
      .token-chip ~ .popover {
        min-width: 250px;
      }
      .token-chip ~ .popover ol {
        list-style-position: outside;
        margin: 0;
        padding-left: 1em;
      }
      .token-chip ~ .popover ol li {
        margin-bottom: 5px;
        font-size: 0.9em;
      }
      .token-chip ~ .popover ol li:only-child, .token-chip ~ .popover ol li:last-child {
        margin-bottom: 0;
      }
      .token-chip ~ .popover ol li::before {
        font-weight: bold;
      }
    </style>
    <div id="veda-bpmn">
      <h4 class="text-center" style="width:100%; position:absolute;z-index:1;top:-0.7em;">
        <span about="@" rel="rdf:type" data-template="v-ui:LabelTemplate"></span>
        <span about="@" rel="bpmn:instanceOf" data-template="v-ui:LabelLinkTemplate"></span>
      </h4>
    </div>
  </div>
`;
