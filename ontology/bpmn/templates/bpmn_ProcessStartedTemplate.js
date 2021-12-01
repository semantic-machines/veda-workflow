export const html = `
<div class="journal-record">
  <hr class="margin-sm">
  <div class="row">
    <div class="col-md-2 col-sm-3 event-type">
      <strong about="@" rel="rdf:type" data-template="v-ui:LabelTemplate"></strong>
    </div>
    <div class="col-md-8 col-sm-6 event-desc">
      <div about="@" rel="bpmn:hasProcessInstance">
        <div>
          <a href="#/@" about="@" property="bpmn:processDefinitionKey"></a>
          <span about="@" rel="bpmn:hasStatus" data-template="v-ui:StatusTemplate"></span>
          <br>
          <span about="@" rel="bpmn:hasStartForm">
            <span>
              <span about="bpmn:StartForm" data-template="v-ui:LabelTemplate"></span>:
              <span about="@" data-template="v-ui:LabelLinkTemplate"></span>,
              <span about="v-s:creator" data-template="v-ui:LabelTemplate"></span>:
              <span about="@" rel="v-s:creator" data-template="v-ui:LabelTemplate"></span>
            </span>
          </span>
        </div>
      </div>
    </div>
    <div class="col-md-2 col-sm-3 event-date text-right">
      <span about="@" property="v-s:created"></span>
    </div>
  </div>
  <div class="sub-journal" rel="v-s:childRecord"></div>
</div>
`;