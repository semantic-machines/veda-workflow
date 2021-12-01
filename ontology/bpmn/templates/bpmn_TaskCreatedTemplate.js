export const html = `
<div class="journal-record margin-md">
  <hr class="margin-sm">
  <div class="row margin-sm">
    <div class="col-md-2 col-sm-3 event-type">
      <span about="@" rel="rdf:type" data-template="v-ui:LabelTemplate"></span>
    </div>
    <div class="col-md-8 col-sm-6 event-desc" rel="bpmn:hasDecisionForm">
      <div class="row">
        <div class="col-sm-6 col-xs-12">
          <strong about="@" data-template="v-ui:LabelLinkTemplate"></strong>
          <i about="@" property="v-s:description"></i>
          <div about="@" rel="v-wf:to" data-template="v-ui:LabelTemplate"></div>
        </div>
        <div class="col-sm-6 col-xs-12" about="@" rel="v-wf:takenDecision">
          <div>
            <strong about="@" property="rdfs:label"></strong>
            <span about="@" rel="v-wf:to">
              <span>
                <span>&rarr;</span>
                <span about="@" data-template="v-ui:LabelTemplate"></span>
              </span>
            </span>
            <i about="@" property="rdfs:comment"></i>
            <div about="@" rel="v-s:creator" data-template="v-ui:LabelTemplate"></div>
            <small about="@" property="v-s:created"></small>
          </div>
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