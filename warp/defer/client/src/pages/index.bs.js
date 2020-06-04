

import * as Curry from "bs-platform/lib/es6/curry.js";
import * as React from "react";
import * as RelayEnv from "../helpers/RelayEnv.bs.js";
import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";
import * as ErrorBoundary from "../components/ErrorBoundary.bs.js";
import * as ReactExperimental from "reason-relay/src/ReactExperimental.bs.js";
import * as PagesQuery_graphql from "../__generated__/pagesQuery_graphql.bs.js";

var convertResponse = PagesQuery_graphql.Internal.convertResponse;

var convertVariables = PagesQuery_graphql.Internal.convertVariables;

var UseQuery = ReasonRelay.MakeUseQuery({
      query: PagesQuery_graphql.node,
      convertResponse: convertResponse,
      convertVariables: convertVariables
    });

var use = UseQuery.use;

var Query_fetch = UseQuery.$$fetch;

var Query_fetchPromised = UseQuery.fetchPromised;

var Query_usePreloaded = UseQuery.usePreloaded;

var Query = {
  Operation: undefined,
  Types: undefined,
  UseQuery: UseQuery,
  use: use,
  $$fetch: Query_fetch,
  fetchPromised: Query_fetchPromised,
  usePreloaded: Query_usePreloaded
};

function Index$Books(Props) {
  var response = Curry._6(use, undefined, undefined, undefined, undefined, undefined, undefined);
  var booksCount = response.books.length;
  console.log("books: ", response.books);
  return React.createElement("div", undefined, React.createElement("h2", {
                  className: "text-4xl font-extrabold tracking-tight text-gray-900 leading-10 sm:text-5xl sm:leading-none md:text-6xl"
                }, "Streaming " + (String(booksCount) + " books....")));
}

var Books = {
  make: Index$Books
};

function Index(Props) {
  var environment = RelayEnv.initEnvironment(undefined);
  return React.createElement("div", {
              className: "flex flex-col items-center justify-center h-full px-4 mx-auto text-center bg-white"
            }, React.createElement(ReasonRelay.Context.Provider.make, Curry._4(ReasonRelay.Context.Provider.makeProps, environment, React.createElement(ErrorBoundary.make, {
                          fallbackComponent: React.createElement("div", undefined, "not found"),
                          children: React.createElement(ReactExperimental.Suspense.make, {
                                children: React.createElement(Index$Books, { }),
                                fallback: React.createElement("div", undefined, "Loading...")
                              })
                        }), undefined, undefined)));
}

var make = Index;

var $$default = Index;

export {
  Query ,
  Books ,
  make ,
  $$default ,
  $$default as default,
  
}
/* UseQuery Not a pure module */
