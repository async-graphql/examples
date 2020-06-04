module ErrorHandler = {
  [@react.component]
  let make = (~fallbackComponent, ~error, ~info) => {
    React.useEffect2(
      () => {
        Js.Console.error3(
          "Something went wrong: ",
          error,
          info.ReasonReactErrorBoundary.componentStack,
        );
        None;
      },
      (error, info),
    );
    fallbackComponent;
  };
};

[@react.component]
let make = (~fallbackComponent, ~children) =>
  <ReasonReactErrorBoundary
    fallback={({error, info}) =>
      <ErrorHandler fallbackComponent error info />
    }>
    children
  </ReasonReactErrorBoundary>;

let default = make;
