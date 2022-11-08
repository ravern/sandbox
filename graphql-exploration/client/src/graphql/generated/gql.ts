/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

const documents = {
    "\n  query HomePage_Posts($limit: Int!, $cursor: String) {\n    posts(limit: $limit, cursor: $cursor) {\n      nextCursor\n      nodes {\n        id\n        title\n        body\n        author {\n          username\n        }\n        likes(limit: 0) {\n          count\n        }\n        comments(limit: 0) {\n          count\n        }\n      }\n    }\n  }\n": types.HomePage_PostsDocument,
    "\n  query PostPage_Post($id: ID!) {\n    post(id: $id) {\n      id\n      title\n      body\n      likes(limit: 0) {\n        count\n      }\n      comments(limit: 1) {\n        count\n        nextCursor\n        nodes {\n          id\n          body\n          author {\n            id\n            username\n          }\n        }\n      }\n    }\n  }\n": types.PostPage_PostDocument,
};

export function graphql(source: "\n  query HomePage_Posts($limit: Int!, $cursor: String) {\n    posts(limit: $limit, cursor: $cursor) {\n      nextCursor\n      nodes {\n        id\n        title\n        body\n        author {\n          username\n        }\n        likes(limit: 0) {\n          count\n        }\n        comments(limit: 0) {\n          count\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query HomePage_Posts($limit: Int!, $cursor: String) {\n    posts(limit: $limit, cursor: $cursor) {\n      nextCursor\n      nodes {\n        id\n        title\n        body\n        author {\n          username\n        }\n        likes(limit: 0) {\n          count\n        }\n        comments(limit: 0) {\n          count\n        }\n      }\n    }\n  }\n"];
export function graphql(source: "\n  query PostPage_Post($id: ID!) {\n    post(id: $id) {\n      id\n      title\n      body\n      likes(limit: 0) {\n        count\n      }\n      comments(limit: 1) {\n        count\n        nextCursor\n        nodes {\n          id\n          body\n          author {\n            id\n            username\n          }\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query PostPage_Post($id: ID!) {\n    post(id: $id) {\n      id\n      title\n      body\n      likes(limit: 0) {\n        count\n      }\n      comments(limit: 1) {\n        count\n        nextCursor\n        nodes {\n          id\n          body\n          author {\n            id\n            username\n          }\n        }\n      }\n    }\n  }\n"];

export function graphql(source: string): unknown;
export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;