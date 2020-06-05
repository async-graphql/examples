

import * as React from "react";
import * as ReasonReactErrorBoundary from "reason-react/src/ReasonReactErrorBoundary.bs.js";

function ErrorBoundary$ErrorHandler(Props) {
  var fallbackComponent = Props.fallbackComponent;
  var error = Props.error;
  var info = Props.info;
  React.useEffect((function () {
          console.error("Something went wrong: ", error, info.componentStack);
          
        }), /* tuple */[
        error,
        info
      ]);
  return fallbackComponent;
}

var ErrorHandler = {
  make: ErrorBoundary$ErrorHandler
};

function ErrorBoundary(Props) {
  var fallbackComponent = Props.fallbackComponent;
  var children = Props.children;
  return React.createElement(ReasonReactErrorBoundary.make, {
              children: children,
              fallback: (function (param) {
                  return React.createElement(ErrorBoundary$ErrorHandler, {
                              fallbackComponent: fallbackComponent,
                              error: param.error,
                              info: param.info
                            });
                })
            });
}

var make = ErrorBoundary;

var $$default = ErrorBoundary;

export {
  ErrorHandler ,
  make ,
  $$default ,
  $$default as default,
  
}
/* react Not a pure module */
