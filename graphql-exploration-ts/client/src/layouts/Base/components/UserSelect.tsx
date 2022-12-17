import { gql, useApolloClient, useQuery } from "@apollo/client";
import React from "react";
import { UserSelect_UsersDocument } from "../../../graphql/generated/graphql";
import useAsyncCallback from "../../../hooks/useAsyncCallback";

export const USERS_QUERY = gql`
  query UserSelect_Users {
    users {
      id
      username
    }
  }
`;

export default function UserSelect() {
  const client = useApolloClient();

  const { data, loading, error } = useQuery(UserSelect_UsersDocument);
  if (error != null) {
    throw error;
  }

  const handleChange = useAsyncCallback(async (event: React.ChangeEvent<HTMLSelectElement>) => {
    localStorage.setItem("token", event.target.value);
    client.resetStore();
  }, [client]);

  if (loading) {
    return <>Loading..."</>;
  } else if (data != null) {
    return (
      <select onChange={handleChange}>
        {data.users.map(user => (
          <option key={user.id} value={user.id}>{user.username}</option>
        ))}
      </select>
    );
  }
}