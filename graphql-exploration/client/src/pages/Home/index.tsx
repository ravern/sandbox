import { gql, useQuery } from "@apollo/client";
import React from "react";
import { HomePage_PostsDocument } from "../../graphql/generated/graphql";
import useAsyncCallback from "../../hooks/useAsyncCallback";
import CreatePostForm from "./components/CreatePostForm";

import PostItem, { Post } from "./components/PostItem";

const POSTS_QUERY_LIMIT = 9;

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
        isLiked
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
            <PostItem key={post.id} post={post} />
          ))}
        </ul>
        {data.posts.nextCursor != null && <div><button onClick={handleLoadMoreClick}>Load More</button></div>}
        <CreatePostForm />
      </div>
    );
  } else {
    throw new Error("Invalid Apollo state");
  }
}