import { FC } from "react";

import styles from "./style.module.scss";

const CoolCards: FC = () => {
  const meta = [
    {
      url: "https://osu.ppy.sh/beatmapsets/1536948",
      filePath:
        "https://cdn.discordapp.com/attachments/645591556585291776/1038477819912650792/example1.webm",
    },
    {
      url: "https://osu.ppy.sh/beatmapsets/1846040",
      filePath:
        "https://cdn.discordapp.com/attachments/645591556585291776/1038477820751515789/example2.webm",
    },
    {
      url: "https://osu.ppy.sh/beatmapsets/855677",
      filePath:
        "https://cdn.discordapp.com/attachments/645591556585291776/1038477820399210526/example3.webm",
    },
  ];

  return (
    <div className={styles.positioner}>
      <div className={styles.cardWrapper}>
        {meta.map((item, i) => (
          <a
            key={item.url + i}
            href={item.url}
            target={"_blank"}
            rel="noreferrer"
            className={styles.card}
          >
            <video autoPlay muted loop playsInline>
              <source src={item.filePath} type="video/webm" />
            </video>
            <div className={styles.overlay} />
            <div className={styles.shadow} />
          </a>
        ))}
      </div>
    </div>
  );
};
export default CoolCards;
