

import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";

var Types = { };

var responseConverter = ({"__root":{"books":{"f":""}}});

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

var node = ({
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "pagesQuery",
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": "Book",
        "kind": "LinkedField",
        "name": "books",
        "plural": true,
        "selections": [
          {
            "args": null,
            "kind": "FragmentSpread",
            "name": "pages_book"
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Query"
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "pagesQuery",
    "selections": [
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
            "if": null,
            "kind": "Defer",
            "label": "pages_book$defer$pages_comments_book",
            "selections": [
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
                    "name": "text",
                    "storageKey": null
                  },
                  {
                    "alias": null,
                    "args": null,
                    "kind": "ScalarField",
                    "name": "user",
                    "storageKey": null
                  }
                ],
                "storageKey": null
              }
            ]
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "id": null,
    "metadata": {},
    "name": "pagesQuery",
    "operationKind": "query",
    "text": "query pagesQuery {\n  books {\n    ...pages_book\n  }\n}\n\nfragment pages_book on Book {\n  title\n  author\n  ...pages_comments_book @defer(label: \"pages_book$defer$pages_comments_book\")\n}\n\nfragment pages_comments_book on Book {\n  comments {\n    text\n    user\n  }\n}\n"
  }
});

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
