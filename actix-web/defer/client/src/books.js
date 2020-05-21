import React from 'react';
import { Query } from 'react-apollo';
import gql from 'graphql-tag';

const Comments = comments => (
  <ul>
    {comments && comments.comments && comments.comments.map(({ user, text }) => (
      <li key={text}>
        {user}: {text}
      </li>
    ))}
  </ul>
);

const Book = ({ title, author, comments }) => (
  <div>
    <p>{`${title} by ${author}`}</p>
    <Comments comments={comments} />
  </div>
);

const Books = () => (
  <Query
    query={gql`
      {
        books {
          title
          author
          comments @defer {
            user
            text
          }
        }
      }
    `}
  >
    {({ loading, error, data }) => {
      if (loading) return <p>Loading...</p>;
      if (error) return <p>Error :(</p>;

      return data.books.map(book => <Book key={book.title} {...book} />);
    }}
  </Query>
);

export default Books;
