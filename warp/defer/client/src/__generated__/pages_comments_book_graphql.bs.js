

import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";

var Types = { };

var fragmentConverter = ({"__root":{"comments":{"n":""}}});

function convertFragment(v) {
  return ReasonRelay._convertObj(v, fragmentConverter, undefined, undefined);
}

var Internal = {
  fragmentConverter: fragmentConverter,
  fragmentConverterMap: undefined,
  convertFragment: convertFragment
};

var Utils = { };

var node = ({
  "argumentDefinitions": [],
  "kind": "Fragment",
  "metadata": null,
  "name": "pages_comments_book",
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
  ],
  "type": "Book"
});

export {
  Types ,
  Internal ,
  Utils ,
  node ,
  
}
/* fragmentConverter Not a pure module */
