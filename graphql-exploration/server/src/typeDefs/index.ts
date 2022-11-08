import { gql } from 'graphql-tag';

const typeDefs = gql`
  scalar Date

  type Query {
    posts(limit: Int!, cursor: String): Posts!
    post(id: ID!): Post!
  }

  type Mutation {
    createPost(input: CreatePostInput!): Post!
  }

  input CreatePostInput {
    title: String!
    body: String!
  }

  type User {
    id: ID!
    username: String!
    joinedAt: Date!
    bookmarks(limit: Int!, cursor: String): Bookmarks!
    posts(limit: Int!, cursor: String): Posts!
    followers(limit: Int!, cursor: String): Users!
    following(limit: Int!, cursor: String): Users!
  }

  type Post {
    id: ID!
    title: String!
    body: String!
    createdAt: Date!
    author: User!
    likes(limit: Int!, cursor: String): Users!
    comments(limit: Int!, cursor: String): Comments!
  }

  type Comment {
    id: ID!
    body: String!
    createdAt: Date!
    author: User!
    post: Post!
    likes(limit: Int!, cursor: String): Users!
  }

  type Bookmark {
    id: ID!
    createdAt: Date!
    post: Post!
    user: User!
  }

  type Posts {
    count: Int!
    nextCursor: String
    nodes: [Post!]!
  }

  type Comments {
    count: Int!
    nextCursor: String
    nodes: [Comment!]!
  }

  type Bookmarks {
    count: Int!
    nextCursor: String
    nodes: [Bookmark!]!
  }

  type Users {
    count: Int!
    nextCursor: String
    nodes: [User!]!
  }
`;

export default typeDefs;
