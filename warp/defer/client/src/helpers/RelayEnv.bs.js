

import * as RelayHack from "../bindings/RelayHack.bs.js";
import * as Caml_option from "bs-platform/lib/es6/caml_option.js";
import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";
import * as Caml_exceptions from "bs-platform/lib/es6/caml_exceptions.js";

var Graphql_error = Caml_exceptions.create("RelayEnv.Graphql_error");

function initEnvironment(queryRecords) {
  var network = ReasonRelay.Network.makeObservableBased(RelayHack.fetchQuery, undefined, undefined);
  var source = queryRecords !== undefined ? ReasonRelay.RecordSource.make(Caml_option.some(Caml_option.valFromOption(queryRecords)), undefined) : ReasonRelay.RecordSource.make(undefined, undefined);
  var store = ReasonRelay.Store.make(source, undefined, undefined);
  return ReasonRelay.Environment.make(network, store, undefined, undefined, undefined);
}

export {
  Graphql_error ,
  initEnvironment ,
  
}
/* RelayHack Not a pure module */
