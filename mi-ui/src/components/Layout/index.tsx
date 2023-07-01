import React, { FC, ReactNode } from "react";
import { ToastContainer } from "react-toastify";
import { useGlobalTheme } from "@states/theme";
import { useCurrentUser } from "@hooks/useUser";
import Tooltip from "@components/SharedComponents/Tooltip";
import Header from "./Header";

import styles from "./style.module.scss";
import "@fontsource-variable/inter";
import "@fontsource-variable/comfortaa";
import "react-toastify/dist/ReactToastify.css";

type Props = {
  children?: ReactNode;
};

const Layout: FC<Props> = ({ children }) => {
  const user = useCurrentUser(); // Just to fetch the user data

  const { theme } = useGlobalTheme();
  return (
    <>
      {<Header />}
      <main className={styles.contentCenterer}>{children}</main>
      <Tooltip />
      <ToastContainer theme={theme as any} position="bottom-right"  />
    </>
  );
};

export default Layout;
