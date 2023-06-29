import { LoginScreen } from "@components/PageComponents/Home";
import { useUser } from "@hooks/useUser";
import { userData } from "@libs/consts/dummyUserData";
import { NewsType } from "@libs/types/influence";
import { readFileSync } from "fs";
import type { InferGetStaticPropsType, NextPage } from "next";
import dynamic from "next/dynamic";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";

const Home: NextPage<InferGetStaticPropsType<typeof getStaticProps>> = ({
  leaderboard,
  news,
}) => {
  const router = useRouter();
  const { user } = useUser();

  useEffect(() => {
    if (user) router.push("/dashboard");
  }, [user, router]);

  return <LoginScreen topList={leaderboard} newsList={news} />;
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

  const exampleTopList = userData.influences
    .map((influence) => ({
      user: influence.profileData,
      number: Math.floor(Math.random() * 150),
    }))
    .sort((a, b) => b.number - a.number);

  return {
    props: {
      news: exampleNews,
      leaderboard: exampleTopList,
    },
  };
};

export default Home;
