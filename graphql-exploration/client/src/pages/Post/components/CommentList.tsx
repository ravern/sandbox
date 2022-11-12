import { gql, useQuery } from "@apollo/client";
import React from "react";
import { CommentList_PostDocument } from "../../../graphql/generated/graphql";

export const POST_QUERY = gql`
  query CommentList_Post($id: ID!) {
    post(id: $id) {
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

export type CommentListProps = {
  postId: string;
};

export default function CommentList({ postId }: CommentListProps) {
  const { data, loading, error } = useQuery(CommentList_PostDocument, { variables: { id: postId } });
  if (error != null) {
    throw error;
  }

  if (loading) {
    return <>Loading...</>;
  } else if (data != null) {
    return (
      <ul>
        {data.post.comments.nodes.map(comment => (
          <li key={comment.id}>
            <p>{comment.body}</p>
            <p>by {comment.author.username} | {comment.likes.count} likes</p>
            <p>
              <button>Like</button>
            </p>
          </li>
        ))}
      </ul>
    );
  } else {
    throw new Error("Invalid Apollo state");
  }
}