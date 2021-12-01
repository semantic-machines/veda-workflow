import $ from 'jquery';
import veda from '/js/common/veda.js';

export const post = function (individual, template, container, mode, extra) {
  template = $(template);
  container = $(container);

  $("#send", template).click(function() {
    template[0].dispatchEvent(new Event("save"));
  });

  individual["rdfs:label"] = individual["rdf:type"][0]["rdfs:label"];
  if ( mode === "edit" && individual.isNew() ) {
    individual["v-s:controller"] = [ veda.appointment ? veda.appointment : veda.user ];
  }

  function closeModal () {
    var modal = template.closest(".modal");
    if (modal.length) {
      modal.modal("hide");
    }
  }
  individual.one("afterSave afterReset", closeModal);
  template.one("remove", function () {
    individual.off("afterSave", closeModal);
    individual.off("afterReset", closeModal);
  });
};

export const html = `
<div class="container sheet">
  <h3 about="@" property="rdfs:label"></h3>
  <div class="row">
    <div class="col-md-6">
      <em about="v-s:responsible" property="rdfs:label"></em>
      <div rel="v-s:responsible" data-template="v-ui:LabelTemplate" class="view edit -search"></div>
      <veda-control rel="v-s:responsible" data-type="link" class="-view edit search fulltext"></veda-control>
    </div>
    <div class="col-md-6">
      <em about="v-s:dateTo" property="rdfs:label"></em>
      <div property="v-s:dateTo" class="view -edit -search"></div>
      <veda-control property="v-s:dateTo" data-type="dateTime" class="-view edit search"></veda-control>
    </div>
  </div>
  <em about="v-s:description" property="rdfs:label"></em>
  <div property="v-s:description" class="view -edit -search"></div>
  <veda-control property="v-s:description" rows="3" data-type="text" class="-view edit search"></veda-control>
  <em about="v-s:controller" property="rdfs:label"></em>
  <div rel="v-s:controller" data-template="v-ui:LabelTemplate" class="view -edit -search"></div>
  <veda-control rel="v-s:controller" data-type="link" class="-view edit search fulltext"></veda-control>
  <br>
  <div class="actions -view edit -search">
    <button id="send" class="btn btn-warning" about="v-s:Send" property="rdfs:label"></button>
    <span about="@" data-template="v-ui:StandardButtonsTemplate" data-embedded="true" data-buttons="cancel"></span>
  </div>
</div>
`;