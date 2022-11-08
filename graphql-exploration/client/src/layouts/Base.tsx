import React from "react";
import { Outlet } from "react-router-dom";

import UserSelect from "../components/UserSelect";

export default function BaseLayout() {
  return (
    <>
      <div>
        <UserSelect />
      </div>
      <Outlet />
    </>
  );
}