import { nanoid } from 'nanoid';

export type UserType = {
  id: string;
  username: string;
  joinedAt: string;
  followers: string[];
};

export type PostType = {
  id: string;
  title: string;
  body: string;
  createdAt: string;
  authorId: string;
  likes: string[];
};

export type CommentType = {
  id: string;
  body: string;
  createdAt: string;
  authorId: string;
  postId: string;
  likes: string[];
};

export type BookmarkType = {
  id: string;
  createdAt: string;
  postId: string;
  userId: string;
};

const userId1 = nanoid();
const userId2 = nanoid();
const userId3 = nanoid();

const users: { [key: string]: UserType } = {
  [userId1]: {
    id: userId1,
    username: 'dakota',
    joinedAt: new Date().toISOString(),
    followers: [userId2, userId3],
  },
  [userId2]: {
    id: userId2,
    username: 'johnson',
    joinedAt: new Date().toISOString(),
    followers: [userId1],
  },
  [userId3]: {
    id: userId3,
    username: 'kevin',
    joinedAt: new Date().toISOString(),
    followers: [],
  },
};

const postId1 = nanoid();
const postId2 = nanoid();
const postId3 = nanoid();
const postId4 = nanoid();
const postId5 = nanoid();
const postId6 = nanoid();

const posts: { [key: string]: PostType } = {
  [postId1]: {
    id: postId1,
    title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit',
    body: 'Maecenas nec lacus et sem fringilla elementum. Sed dui erat, commodo in auctor et, dapibus non lacus. Aenean ut nulla non ligula pretium varius eu a augue. Ut ac enim lorem. Suspendisse molestie est sit amet ipsum egestas tincidunt. Fusce vitae condimentum dolor, nec ultrices arcu. Aenean eu rhoncus massa. Sed quis vulputate felis. Suspendisse sit amet porta est. Proin sed risus ac ante rhoncus finibus sit amet eget nisi.',
    createdAt: new Date().toISOString(),
    authorId: userId1,
    likes: [],
  },
  [postId2]: {
    id: postId2,
    title: 'Pellentesque pretium id nisl quis pulvinar',
    body: 'Fusce dapibus ornare eros, ut vehicula augue tempor quis. Donec malesuada fermentum fringilla. Mauris massa mi, interdum et lorem ac, mollis vulputate elit. Nam cursus finibus massa. Fusce rhoncus sed lectus et ultrices. Mauris tincidunt sem id ligula congue, a egestas libero vestibulum. Nullam porta sem turpis, vitae convallis metus molestie sit amet.',
    createdAt: new Date().toISOString(),
    authorId: userId1,
    likes: [],
  },
  [postId3]: {
    id: postId3,
    title: 'Cras consectetur orci et ante gravida euismod',
    body: 'Ut non mauris sit amet nunc ultricies gravida. Donec mollis diam non diam posuere, nec semper nunc vestibulum. Pellentesque eget venenatis risus. Nulla hendrerit dui quis rhoncus tincidunt. Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
    createdAt: new Date().toISOString(),
    authorId: userId2,
    likes: [userId3],
  },
  [postId4]: {
    id: postId4,
    title: 'Curabitur non sodales risus. Donec auctor finibus lacus nec imperdiet',
    body: 'Integer consectetur leo sit amet lacus volutpat mollis. Quisque malesuada, nisl vel bibendum sollicitudin, nisi sapien dapibus nibh, sit amet bibendum ligula dolor ac ex. Nunc auctor lorem eget sollicitudin ullamcorper. Suspendisse eget odio eget eros blandit consectetur a at augue. Fusce non orci efficitur, efficitur leo ut, vestibulum leo. Sed non felis pulvinar est pretium tempor.',
    createdAt: new Date().toISOString(),
    authorId: userId2,
    likes: [userId1],
  },
  [postId5]: {
    id: postId5,
    title: 'Morbi id lacus id nibh blandit viverra quis quis leo.',
    body: 'Cras ac condimentum magna. Quisque quam dui, consectetur a leo sed, auctor ultrices mi. Vivamus pellentesque, lorem nec convallis volutpat, sapien purus cursus odio, at sodales massa massa a erat. Sed ac nisi et lectus tincidunt vehicula eget id libero. Maecenas lacinia commodo diam, efficitur tristique nunc. In scelerisque euismod felis vitae bibendum. Sed gravida elit leo, in tristique sapien egestas a. Morbi ut suscipit dui. Sed facilisis elit sit amet ex sagittis, vitae vulputate neque volutpat.',
    createdAt: new Date().toISOString(),
    authorId: userId2,
    likes: [],
  },
  [postId6]: {
    id: postId6,
    title: 'Quisque faucibus venenatis sapien, quis aliquam magna tincidunt at',
    body: 'Nulla dictum enim ac lorem mollis pellentesque. Etiam et velit sed mauris pellentesque consectetur euismod id sem. Cras sagittis suscipit nisi id scelerisque. Pellentesque hendrerit ipsum vitae sodales maximus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Aliquam quam arcu, aliquet sit amet rhoncus quis, malesuada hendrerit erat.',
    createdAt: new Date().toISOString(),
    authorId: userId3,
    likes: [userId1, userId2],
  },
};

const comments: { [key: string]: CommentType } = {};

const bookmarks: { [key: string]: BookmarkType } = {};

const data = { users, posts, comments, bookmarks };

export default data;
