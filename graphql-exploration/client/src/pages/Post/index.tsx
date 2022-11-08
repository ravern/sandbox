import { gql, useQuery } from "@apollo/client";
import React from "react";
import { useParams } from "react-router-dom";
import { PostPage_PostDocument } from "../../graphql/generated/graphql";

export const POST_QUERY = gql`
  query PostPage_Post($id: ID!) {
    post(id: $id) {
      id
      title
      body
      likes(limit: 0) {
        count
      }
      comments(limit: 1) {
        count
        nextCursor
        nodes {
          id
          body
          author {
            id
            username
          }
        }
      }
    }
  }
`;

export default function PostPage() {
  const { postId } = useParams();
  const { data, error } = useQuery(PostPage_PostDocument, { variables: { id: postId } });
  if (error) {
    throw error;
  }
  
  return <>{JSON.stringify(data)}</>;
}