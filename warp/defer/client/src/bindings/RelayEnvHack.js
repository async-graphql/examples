import fetchMultipart from 'fetch-multipart-graphql';
import { Observable } from 'relay-runtime';

export function fetchQuery(operation, variables) {
  return Observable.create((sink) => {
    fetchMultipart('http://localhost:9000', {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        accept: 'application/json',
      },
      body: JSON.stringify({
        query: operation.text,
        variables,
      }),
      credentials: 'same-origin',
      onNext: (parts) => {
        sink.next(parts);
      },
      onError: (err) => sink.error(err),
      onComplete: () => sink.complete(),
    });
  });
}
