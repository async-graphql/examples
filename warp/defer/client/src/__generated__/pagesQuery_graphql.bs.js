

import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";

var Types = { };

var responseConverter = ({});

function convertResponse(v) {
  return ReasonRelay._convertObj(v, responseConverter, undefined, undefined);
}

var variablesConverter = ({});

function convertVariables(v) {
  return ReasonRelay._convertObj(v, variablesConverter, undefined, undefined);
}

var Internal = {
  responseConverter: responseConverter,
  responseConverterMap: undefined,
  convertResponse: convertResponse,
  variablesConverter: variablesConverter,
  variablesConverterMap: undefined,
  convertVariables: convertVariables
};

var Utils = { };

var node = ((function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "concreteType": "Book",
    "kind": "LinkedField",
    "name": "books",
    "plural": true,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "title",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "author",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "concreteType": "Comment",
        "kind": "LinkedField",
        "name": "comments",
        "plural": true,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "user",
            "storageKey": null
          },
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "text",
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "pagesQuery",
    "selections": (v0/*: any*/),
    "type": "Query"
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "pagesQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "id": null,
    "metadata": {},
    "name": "pagesQuery",
    "operationKind": "query",
    "text": "query pagesQuery {\n  books {\n    title\n    author\n    comments @defer {\n      user\n      text\n    }\n  }\n}\n"
  }
};
})());

var include = ReasonRelay.MakePreloadQuery({
      query: node,
      convertVariables: convertVariables
    });

var preload = include.preload;

var preloadTokenToObservable = include.preloadTokenToObservable;

var preloadTokenToPromise = include.preloadTokenToPromise;

export {
  Types ,
  Internal ,
  Utils ,
  node ,
  preload ,
  preloadTokenToObservable ,
  preloadTokenToPromise ,
  
}
/* responseConverter Not a pure module */
