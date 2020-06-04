[@bs.module "./RelayEnvHack"]
external fetchQuery:
  (ReasonRelay.Network.operation, Js.Json.t, ReasonRelay.cacheConfig) =>
  ReasonRelay.Observable.t =
  "fetchQuery";
