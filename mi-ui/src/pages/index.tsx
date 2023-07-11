import { LoginScreen } from "@components/PageComponents/Home";
import { useCurrentUser } from "@hooks/useUser";
import { NewsType } from "@libs/types/influence";
import { readFileSync } from "fs";
import type { InferGetStaticPropsType, NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { toast } from "react-toastify";

const Home: NextPage<InferGetStaticPropsType<typeof getStaticProps>> = ({
  leaderboard,
  news,
}) => {
  const router = useRouter();
  const { user, isLoading } = useCurrentUser();

  useEffect(() => {
    //if (!user && isLoading) toast.loading("Logging in...");
    if (user) router.push("/dashboard");
  }, [user, router, isLoading]);

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

export default Home;
