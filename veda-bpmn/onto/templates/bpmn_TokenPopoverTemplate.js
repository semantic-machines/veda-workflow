export const html = `
  <div>
    <span about="@" data-template="v-ui:LabelLinkTemplate"></span>
    <ol rel="bpmn:hasWorkOrder">
      <li>
        <div about="@" rel="bpmn:hasExecutor">
          <div><strong about="bpmn:hasExecutor" property="rdfs:label"></strong> <span about="@" data-template="v-ui:LabelLinkTemplate"></span></div>
        </div>
        <div about="@" rel="bpmn:hasDecisionForm">
          <div><strong about="bpmn:hasDecisionForm" property="rdfs:label"></strong><span about="@" data-template="v-ui:LabelLinkTemplate"></span></div>
        </div>
      </li>
    </ol>
  </div>
`;
