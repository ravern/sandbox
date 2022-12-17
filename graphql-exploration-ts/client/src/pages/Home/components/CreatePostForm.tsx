import { gql, useMutation, useQuery } from "@apollo/client";
import { nanoid } from "nanoid";
import React, { ReactText, useCallback, useState } from "react";
import { CreatePostForm_CreatePostDocument, CreatePostForm_CurrentUserDocument, HomePage_PostsDocument } from "../../../graphql/generated/graphql";
import useAsyncCallback from "../../../hooks/useAsyncCallback";

export const CURRENT_USER_QUERY = gql`
  query CreatePostForm_CurrentUser {
    currentUser {
      id
      username
    }
  }
`;

export const CREATE_POST_MUTATION = gql`
  mutation CreatePostForm_CreatePost($input: CreatePostInput!) {
    createPost(input: $input) {
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
`;

export default function CreatePostForm() {
  const { data } = useQuery(CreatePostForm_CurrentUserDocument);

  const [createPost] = useMutation(CreatePostForm_CreatePostDocument, {
    refetchQueries: [
      "HomePage_Posts",
    ],
  });

  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");

  const handleTitleChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    setTitle(event.target.value);
  }, [setTitle]);

  const handleBodyChange = useCallback((event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setBody(event.target.value);
  }, [setBody]);

  const handleSubmit = useAsyncCallback(async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    await createPost({
      variables: {
        input: {
          title,
          body,
        }
      }
    })
  }, [createPost, title, body]);

  return (
    <form onSubmit={handleSubmit}>
      <h1>Create a new post</h1>
      <div>
        <label htmlFor="title">Title</label>
        <div>
          <input name="title" value={title} onChange={handleTitleChange} />
        </div>
      </div>
      <div>
        <label htmlFor="body">Body</label>
        <div>
          <textarea name="body" onChange={handleBodyChange} />
        </div>
      </div>
      <button type="submit">Create Post</button>
    </form>
  );
}