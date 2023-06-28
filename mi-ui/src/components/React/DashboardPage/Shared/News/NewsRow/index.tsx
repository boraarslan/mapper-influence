import { FC, useState, lazy, Suspense } from "react";
import type { NewsType } from "src/libs/types/influence";
import Modal from "@components/React/Modal";

import styles from "./style.module.scss";

const LazyMarkdown = lazy(() => import("./RenderMarkdown"));

const NewsRow: FC<NewsType> = ({ fullText, title, date, desc }) => {
  const [showModal, setShowModal] = useState(false);

  return (
    <>
      {showModal && (
        <Modal showModal={showModal} setShowModal={setShowModal}>
          <Suspense fallback={<h4>Loading...</h4>}>
            <LazyMarkdown mdString={fullText} />
          </Suspense>
        </Modal>
      )}
      <div className={styles.newsRow} onClick={() => setShowModal(true)}>
        <span className={styles.type}>{date}</span>
        <h4>{title}</h4>
        <p>{desc}</p>
      </div>
    </>
  );
};

export default NewsRow;
