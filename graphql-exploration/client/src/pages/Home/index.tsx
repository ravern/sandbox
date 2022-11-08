import { gql, useQuery } from "@apollo/client";
import React, { useCallback } from "react";
import { Link } from "react-router-dom";
import { HomePage_PostsDocument } from "../../graphql/generated/graphql";
import useAsyncCallback from "../../hooks/useAsyncCallback";

const POSTS_QUERY_LIMIT = 2;

export const POSTS_QUERY = gql`
  query HomePage_Posts($limit: Int!, $cursor: String) {
    posts(limit: $limit, cursor: $cursor) {
      nextCursor
      nodes {
        id
        title
        body
        author {
          id
          username
        }
        likes(limit: 0) {
          count
        }
        comments(limit: 0) {
          count
        }
      }
    }
  }
`;

export default function HomePage() {
  const { data, loading, error, fetchMore } = useQuery(HomePage_PostsDocument, { variables: { limit: POSTS_QUERY_LIMIT } });
  if (error != null) {
    throw error;
  }

  const handleLoadMoreClick = useAsyncCallback(async () => {
    if (data?.posts.nextCursor != null) {
      await fetchMore({
        variables: {
          limit: POSTS_QUERY_LIMIT,
          cursor: data?.posts.nextCursor,
        }
      })
    }
  }, [data, fetchMore]);

  if (loading) {
    return <>Loading...</>;
  } else if (data != null) {
    return (
      <div>
        <ul>
          {data.posts.nodes.map(post => (
            <li key={post.id}>
              <h2>{post.title}</h2>
              <p>{post.body}</p>
              <p>by {post.author.username} | {post.likes.count} likes | {post.comments.count} comments</p>
              <p>
                <button>Like</button>
                <Link to={`/posts/${post.id}`}>View Comments</Link>
              </p>
            </li>
          ))}
        </ul>
        {data.posts.nextCursor != null && <div><button onClick={handleLoadMoreClick}>Load More</button></div>}
      </div>
    );
  } else {
    throw new Error("Invalid Apollo state");
  }
}