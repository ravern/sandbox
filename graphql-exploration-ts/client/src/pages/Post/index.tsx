import { gql, useQuery } from "@apollo/client";
import React from "react";
import { useParams } from "react-router-dom";

import { PostPage_PostDocument } from "../../graphql/generated/graphql";
import CommentList from "./components/CommentList";

export const POST_QUERY = gql`
  query PostPage_Post($id: ID!) {
    post(id: $id) {
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
      comments(limit: 1) {
        count
        nextCursor
        nodes {
          id
          body
          likes(limit: 0) {
            count
          }
          author {
            id
            username
          }
        }
      }
    }
  }
`;

export const LIKE_MUTATION = gql`
  mutation PostPage_LikePost($input: LikePostInput!) {
    likePost(input: $input) {
      id
      likes(limit: 0) {
        count
      }
      isLiked
    }
  }
`;

export default function PostPage() {
  const { postId } = useParams();

  const { data, loading, error } = useQuery(PostPage_PostDocument, { variables: { id: postId } });
  if (error != null) {
    throw error;
  }
  
  if (loading) {
    return <>Loading...</>;
  } else if (data != null) {
    return (
      <>
        <h2>{data.post.title}</h2>
        <p>{data.post.body}</p>
        <p>by {data.post.author.username} | {data.post.likes.count} likes | {data.post.comments.count} comments</p>
        <p>
          <button>Like</button>
        </p>
        <CommentList postId={postId} />
      </>
    ); 
  } else {
    throw new Error("Invalid Apollo state");
  }
}