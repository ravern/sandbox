import { useApolloClient } from "@apollo/client";
import React from "react";
import useAsyncCallback from "../hooks/useAsyncCallback";

const users = [
  {id: "awfwea", username: "widoko"},
  {id: "awfweer3r4", username: "bidojijhi"},
  {id: "awfweawefa", username: "grfaweidoko"},
]

export default function UserSelect() {
  const client = useApolloClient();

  const handleChange = useAsyncCallback(async () => {
    client.resetStore();
  }, [client]);

  return (
    <select onChange={handleChange}>
      {users.map(user => (
        <option value={user.id}>{user.username}</option>
      ))}
    </select>
  );
}