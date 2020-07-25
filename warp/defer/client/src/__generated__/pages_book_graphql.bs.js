

import * as ReasonRelay from "reason-relay/src/ReasonRelay.bs.js";

var Types = { };

var fragmentConverter = ({"__root":{"":{"f":""}}});

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
  "name": "pages_book",
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
      "kind": "Defer",
      "selections": [
        {
          "args": null,
          "kind": "FragmentSpread",
          "name": "pages_comments_book"
        }
      ]
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
