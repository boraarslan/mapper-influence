import React, { FC, ReactNode } from "react";
import Header from "./Header";

import "@fontsource-variable/inter";
import "@fontsource-variable/comfortaa";
import Tooltip from "@components/SharedComponents/Tooltip";
import styles from "./style.module.scss";
import { useUser } from "@hooks/useUser";

type Props = {
  children?: ReactNode;
};

const Layout: FC<Props> = ({ children }) => {
  const user = useUser();

  console.log(user);

  return (
    <>
      {<Header />}
      <main className={styles.contentCenterer}>{children}</main>
      <Tooltip />
    </>
  );
};

export default Layout;
