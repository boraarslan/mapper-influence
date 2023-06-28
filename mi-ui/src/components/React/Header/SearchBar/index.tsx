import { FC, useRef, useState } from "react";
import AwesomeDebouncePromise from "awesome-debounce-promise";
import { useOnClickOutside } from "usehooks-ts";

import Results from "./Results";
import type { UserBase } from "src/libs/types/user";
import { MaxNameLength } from "src/libs/consts";
import Magnify from "@components/React/Svg/Magnify";

import styles from "./styles.module.scss";

type Props = {
  className?: string;
};

const SearchBar: FC<Props> = ({ className }) => {
  const containerRef = useRef(null);
  const [results, setResults] = useState<UserBase[]>([]);
  const [showResults, setShowResults] = useState(false);

  useOnClickOutside(containerRef, () => setShowResults(false));

  const searchUser = (query: string) => {
    setResults(
      Array.from(Array(10).keys()).map((_, index) => ({
        username: query,
        avatarUrl: "https://picsum.photos/200",
        id: index,
        flag: { code: "TR", name: "Türkiye" },
      }))
    );
    // TODO: Search user service
  };

  const debouncedSearch = AwesomeDebouncePromise(searchUser, 500);

  const handleChange = (query: string) => {
    // Hide results element if query is empty
    setShowResults(!!query);
    // TODO: Display loading indicator

    debouncedSearch(query);
  };

  const wrapperClass = `${styles.searchBorder} ${className}`;

  return (
    <div
      className={wrapperClass}
      ref={containerRef}
    >
      <div className={styles.searchBar}>
        <input
          onChange={(e) => handleChange(e.target.value)}
          placeholder={"Search User"}
          maxLength={MaxNameLength}
        />
        <button className={styles.magnifyButton}>
          <Magnify className={styles.magnifySvg} />
        </button>
      </div>
      {showResults && <Results results={results} />}
    </div>
  );
};

export default SearchBar;
