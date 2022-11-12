import { ApolloServerErrorCode } from '@apollo/server/errors';
import { GraphQLError } from 'graphql';
import { nanoid } from 'nanoid';

import db, { BookmarkType, CommentType, PostType, UserType } from '../db';
import { Resolvers } from '../generated/graphql';
import getObjects from './helpers/getObjects';
import paginatedResolver from './helpers/paginatedResolver';

const resolvers: Resolvers = {
  Query: {
    users: async () => {
      return Object.values(db.users);
    },
    currentUser: async (_parent, _args, { userId }) => {
      if (userId == null) {
        throw new GraphQLError('You are not logged in!', {
          extensions: {
            code: 'UNAUTHENTICATED',
          },
        });
      }
      return db.users[userId];
    },
    post: async (_parent, { id }) => {
      const post = db.posts[id];
      if (post == null) {
        throw new GraphQLError('Post not found.', {
          extensions: { code: ApolloServerErrorCode.BAD_USER_INPUT },
        });
      }
      return post;
    },
    posts: paginatedResolver<{}, PostType>(() => db.posts),
  },
  Mutation: {
    createPost: async (_parent, { input: { title, body } }, { userId }) => {
      if (userId == null) {
        throw new GraphQLError('You are not logged in!', {
          extensions: {
            code: 'UNAUTHENTICATED',
          },
        });
      }
      const post = {
        id: nanoid(),
        title,
        body,
        createdAt: new Date().toISOString(),
        authorId: userId,
        likes: [],
      };
      db.posts[post.id] = post;
      return post;
    },
    likePost: async (_parent, { input: { id, like } }, { userId }) => {
      if (userId == null) {
        throw new GraphQLError('You are not logged in!', {
          extensions: {
            code: 'UNAUTHENTICATED',
          },
        });
      }
      const post = db.posts[id];
      if (post == null) {
        throw new GraphQLError('Post not found.', {
          extensions: { code: ApolloServerErrorCode.BAD_USER_INPUT },
        });
      }

      post.likes = post.likes.filter(id => id !== userId);
      if (like) {
        post.likes.push(userId);
      }

      await new Promise(resolve => setTimeout(resolve, 5000));

      return post;
    },
  },
  User: {
    bookmarks: paginatedResolver<UserType, BookmarkType>(user =>
      getObjects<BookmarkType>(db.bookmarks, bookmark => bookmark.userId === user.id),
    ),
    posts: paginatedResolver<UserType, PostType>(user =>
      getObjects<PostType>(db.posts, post => post.authorId === user.id),
    ),
    following: paginatedResolver<UserType, UserType>(user =>
      getObjects<UserType>(db.users, followedUser => followedUser.followers.includes(user.id)),
    ),
    followers: paginatedResolver<UserType, UserType>(user =>
      getObjects<UserType>(db.users, followingUser => user.followers.includes(followingUser.id)),
    ),
  },
  Post: {
    author: async post => {
      return db.users[post.authorId];
    },
    likes: paginatedResolver<PostType, UserType>(post =>
      getObjects<UserType>(db.users, user => post.likes.includes(user.id)),
    ),
    comments: paginatedResolver<PostType, CommentType>(post =>
      getObjects<CommentType>(db.comments, comment => comment.postId === post.id),
    ),
    isLiked: async (post, _args, { userId }) => {
      return post.likes.includes(userId);
    },
  },
  Comment: {
    author: async post => {
      return db.users[post.authorId];
    },
    likes: paginatedResolver<CommentType, UserType>(comment =>
      getObjects<UserType>(db.users, user => comment.likes.includes(user.id)),
    ),
    isLiked: async (comment, _args, { userId }) => {
      return comment.likes.includes(userId);
    },
  },
  Bookmark: {
    post: async bookmark => {
      return db.posts[bookmark.postId];
    },
    user: async bookmark => {
      return db.users[bookmark.userId];
    },
  },
};

export default resolvers;
