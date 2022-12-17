import React from "react";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";

import HomePage from "../pages/Home";
import BaseLayout from "../layouts/Base";
import PostPage from "../pages/Post";
import { ApolloProvider } from "@apollo/client";
import client from "../graphql";

const router = createBrowserRouter([
  {
    path: "/",
    element: <BaseLayout />,
    children: [
      {
        path: "",
        element: <HomePage />,
      },
      {
        path: "posts/:postId",
        element: <PostPage />,
      },
    ]
  },
]);

export default function App() {
  return (
    <ApolloProvider client={client}>
      <RouterProvider router={router} />
    </ApolloProvider>
  );
}