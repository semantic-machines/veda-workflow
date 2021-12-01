export const html = `
<div class="journal-record margin-md">
  <hr class="margin-sm">
  <div class="row margin-sm">
    <div class="col-md-2 col-sm-3 event-type">
      <span about="@" rel="rdf:type" data-template="v-ui:LabelTemplate"></span>
    </div>
    <div class="col-md-8 col-sm-6 event-desc">
      <div about="@" rel="bpmn:hasProcessInstance">
        <span about="@" property="bpmn:processDefinitionKey"></span>
      </div>
    </div>
    <div class="col-md-2 col-sm-3 event-date text-right">
      <span about="@" property="v-s:created"></span>
    </div>
  </div>
</div>
`;