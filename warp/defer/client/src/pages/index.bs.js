

import * as Curry from "bs-platform/lib/es6/curry.js";
import * as React from "react";
import * as RelayEnv from "../helpers/RelayEnv.bs.js";
import * as Belt_Array from "bs-platform/lib/es6/belt_Array.js";
import * as Belt_Option from "bs-platform/lib/es6/belt_Option.js";
import * as Caml_option from "bs-platform/lib/es6/caml_option.js";
import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";
import * as ErrorBoundary from "../components/ErrorBoundary.bs.js";
import * as ReactExperimental from "reason-relay/src/ReactExperimental.bs.js";
import * as PagesQuery_graphql from "../__generated__/pagesQuery_graphql.bs.js";
import * as Pages_book_graphql from "../__generated__/pages_book_graphql.bs.js";
import * as Pages_comments_book_graphql from "../__generated__/pages_comments_book_graphql.bs.js";

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

var convertFragment = Pages_comments_book_graphql.Internal.convertFragment;

var UseFragment = ReasonRelay.MakeUseFragment({
      fragmentSpec: Pages_comments_book_graphql.node,
      convertFragment: convertFragment
    });

function use$1(fRef) {
  return Curry._1(UseFragment.use, fRef);
}

function useOpt(opt_fRef) {
  return Curry._1(UseFragment.useOpt, opt_fRef !== undefined ? Caml_option.some(Caml_option.valFromOption(opt_fRef)) : undefined);
}

var BookCommentsFragment = {
  Operation: undefined,
  Types: undefined,
  UseFragment: UseFragment,
  use: use$1,
  useOpt: useOpt
};

function Index$Comments(Props) {
  var bookRef = Props.bookRef;
  var book = Curry._1(UseFragment.use, bookRef);
  return Belt_Array.map(Belt_Option.getWithDefault(book.comments, []), (function (comment) {
                var text = comment.text;
                var user = comment.user;
                return React.createElement("li", {
                            key: text
                          }, "" + (String(user) + (": " + (String(text) + ""))));
              }));
}

var Comments = {
  make: Index$Comments
};

var convertFragment$1 = Pages_book_graphql.Internal.convertFragment;

var UseFragment$1 = ReasonRelay.MakeUseFragment({
      fragmentSpec: Pages_book_graphql.node,
      convertFragment: convertFragment$1
    });

function use$2(fRef) {
  return Curry._1(UseFragment$1.use, fRef);
}

function useOpt$1(opt_fRef) {
  return Curry._1(UseFragment$1.useOpt, opt_fRef !== undefined ? Caml_option.some(Caml_option.valFromOption(opt_fRef)) : undefined);
}

var BookFragment = {
  Operation: undefined,
  Types: undefined,
  UseFragment: UseFragment$1,
  use: use$2,
  useOpt: useOpt$1
};

function Index$Book(Props) {
  var bookRef = Props.bookRef;
  var book = Curry._1(UseFragment$1.use, bookRef);
  var title = book.title;
  var author = book.author;
  return React.createElement("div", undefined, React.createElement("p", undefined, "" + (String(title) + (" by " + (String(author) + "")))), React.createElement(Index$Comments, {
                  bookRef: Curry._1(book.getFragmentRefs, undefined)
                }));
}

var Book = {
  make: Index$Book
};

function Index$Books(Props) {
  var response = Curry._6(use, undefined, undefined, undefined, undefined, undefined, undefined);
  var booksCount = response.books.length;
  return React.createElement("div", undefined, React.createElement("h2", {
                  className: "text-4xl font-extrabold tracking-tight text-gray-900 leading-10 sm:text-5xl sm:leading-none md:text-6xl pb-10"
                }, "Streaming " + (String(booksCount) + " books....")), Belt_Array.mapWithIndex(response.books, (function (idx, book) {
                    return React.createElement(Index$Book, {
                                bookRef: Curry._1(book.getFragmentRefs, undefined),
                                key: String(idx)
                              });
                  })));
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
  BookCommentsFragment ,
  Comments ,
  BookFragment ,
  Book ,
  Books ,
  make ,
  $$default ,
  $$default as default,
  
}
/* UseQuery Not a pure module */
