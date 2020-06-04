exception Graphql_error(string);

let initEnvironment =
    (~queryRecords: option(ReasonRelay.recordSourceRecords)) => {
  let network =
    ReasonRelay.Network.makeObservableBased(
      ~observableFunction=RelayHack.fetchQuery,
      (),
    );
  let source =
    switch (queryRecords) {
    | Some(queryRecords) =>
      ReasonRelay.RecordSource.make(~records=queryRecords, ())
    | None => ReasonRelay.RecordSource.make()
    };
  let store = ReasonRelay.Store.make(~source, ());

  ReasonRelay.Environment.make(~network, ~store, ());
};
