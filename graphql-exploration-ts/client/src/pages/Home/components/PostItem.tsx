import { gql, useMutation } from "@apollo/client";
import React from "react";
import { Link } from "react-router-dom";

import { HomePage_PostsQuery, PostItem_LikePostDocument } from "../../../graphql/generated/graphql";
import useAsyncCallback from "../../../hooks/useAsyncCallback";

export const LIKE_MUTATION = gql`
  mutation PostItem_LikePost($input: LikePostInput!) {
    likePost(input: $input) {
      id
      likes(limit: 0) {
        count
      }
      isLiked
    }
  }
`;

export type Post = HomePage_PostsQuery["posts"]["nodes"][number];

export type PostItemProps = {
  post: Post;
};

export default function PostItem({ post }) {
  const [likePost] = useMutation(PostItem_LikePostDocument, {
    optimisticResponse: ({ input: { id, like } }) => {
      let likesCount = post.likes.count;
      if (post.isLiked && !like) {
        likesCount -= 1;
      } else if (!post.isLiked && like) {
        likesCount += 1;
      }
      return {
        __typename: "Mutation" as const,
        likePost: {
          __typename: "Post" as const,
          id,
          likes: {
            __typename: "Users" as const,
            count: likesCount,
          },
          isLiked: like,
        }
      };
    }
  });

  const handleLikeClick = useAsyncCallback(async () => {
    await likePost({
      variables: {
        input: {
          id: post.id,
          like: !post.isLiked,
        }
      }
    });
  }, [likePost, post]);

  return (
    <li>
      <h2>{post.title}</h2>
      <p>{post.body}</p>
      <p>by {post.author.username} | {post.likes.count} likes | {post.comments.count} comments</p>
      <p>
        <button onClick={handleLikeClick}>{post.isLiked ? "Unlike" : "Like"}</button>
        <Link to={`/posts/${post.id}`}>View Comments</Link>
      </p>
    </li>
  );
}