import { FC, useCallback, useRef, useState } from "react";
import { useOnClickOutside } from "usehooks-ts";
import AwesomeDebouncePromise from "awesome-debounce-promise";
import { Magnify } from "@components/SvgComponents";
import { MaxNameLength } from "@libs/consts";
import { UserBase } from "@libs/types/user";
import Results from "./Results";

import styles from "./styles.module.scss";

type Props = {
  className?: string;
};

const SearchBar: FC<Props> = ({ className }) => {
  const containerRef = useRef(null);
  const [results, setResults] = useState<UserBase[]>([]);
  const [showResults, setShowResults] = useState(false);

  useOnClickOutside(containerRef, () => setShowResults(false));

  const searchUser = useCallback((query: string) => {
    setResults(
      Array.from(Array(10).keys()).map((_, index) => ({
        username: query,
        avatarUrl: "https://picsum.photos/200",
        id: index,
        flag: { code: "TR", name: "Türkiye" },
      }))
    );
    // TODO: Search user service
  }, []);

  const debouncedSearch = AwesomeDebouncePromise(searchUser, 500);

  const handleChange = (query: string) => {
    // Hide results element if query is empty
    setShowResults(!!query);
    // TODO: Display loading indicator

    debouncedSearch(query);
  };

  const wrapperClass = `${styles.searchBorder} ${className}`;

  return (
    <div className={wrapperClass} ref={containerRef}>
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
