import { useEffect, useState } from "react";
import type { NextPage, InferGetStaticPropsType } from "next";
import { useRouter } from "next/router";
import dynamic from "next/dynamic";
import { readFileSync } from "fs";
import { DUMMY_USER } from "@libs/consts/dummyUserData";
import { NewsType } from "@libs/types/influence";
import { useCurrentUser } from "@hooks/useUser";

const DynamicNewsScreen = dynamic(() =>
  import("@components/PageComponents/Home").then((r) => r.NewsScreen)
);

const DynamicTutorialScreen = dynamic(() =>
  import("@components/PageComponents/Home").then((r) => r.TutorialScreen)
);

const Dashboard: NextPage<InferGetStaticPropsType<typeof getStaticProps>> = ({
  leaderboard,
  news,
}) => {
  const router = useRouter();
  const [screen, setScreen] = useState<"Tutorial" | "News">("Tutorial");
  const { user, isLoading } = useCurrentUser();

  const [isHydrated, setIsHydrated] = useState(false);
  useEffect(() => {
    if (!isHydrated) setIsHydrated(true);
    if (!user && isHydrated && !isLoading) router.push("/");
  }, [user, isHydrated, router, isLoading]);

  switch (screen) {
    case "News":
      return <DynamicNewsScreen newsList={news} topList={leaderboard} />;
    case "Tutorial":
      return (
        <DynamicTutorialScreen>
          <button onClick={() => setScreen("News")}>Close tutorial</button>
        </DynamicTutorialScreen>
      );
  }
};

export const getStaticProps = async () => {
  const file = readFileSync("src/libs/consts/exampleChangelog.md", "utf-8");

  const exampleNews: NewsType[] = [
    {
      fullText: file,
      title: "Version 1.0 is out!",
      date: new Date().toDateString(),
      desc: "Not really. This is just a placeholder.",
    },
  ];

  /*
  const exampleTopList = DUMMY_USER.influences
    .map((influence) => ({
      user: influence.profileData,
      number: Math.floor(Math.random() * 150),
    }))
    .sort((a, b) => b.number - a.number);
  */

  return {
    props: {
      news: exampleNews,
      leaderboard: [],
    },
  };
};

export default Dashboard;
