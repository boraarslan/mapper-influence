import { BeatmapType } from "@libs/types/user";
import { FeaturedMapsResponse, UserFullResponse } from "@services/user";

const exampleMapList: FeaturedMapsResponse[] = [
  {
    featured_map_id: 3535240,
    beatmapset: {
      id: 1801710,
      status: BeatmapType.Graveyard,
      creator: "Fursum",
      names: {
        artist: "NCT",
        artist_unicode: "NCT",
        title: "Before I Go (Blooom remix)",
        title_unicode: "Before I Go (Blooom remix)",
      },
      covers: {
        card: "https://assets.ppy.sh/beatmaps/1801710/covers/card.jpg",
        cover: "https://assets.ppy.sh/beatmaps/1801710/covers/cover.jpg",
        list: "https://assets.ppy.sh/beatmaps/1801710/covers/list.jpg",
        slimcover:
          "https://assets.ppy.sh/beatmaps/1801710/covers/slimcover.jpg",
      },

      beatmaps: [
        {
          difficulty_rating: 5,
          id: 3535240,
          name: "pjm",
          url: "https://osu.ppy.sh/beatmapsets/1729824#osu/3535240",
        },
      ],
    },
  },
  {
    featured_map_id: 3535240,
    beatmapset: {
      id: 1801710,
      status: BeatmapType.Graveyard,
      creator: "Fursum",
      names: {
        artist: "NCT",
        artist_unicode: "NCT",
        title: "Before I Go (Blooom remix)",
        title_unicode: "Before I Go (Blooom remix)",
      },
      covers: {
        card: "https://assets.ppy.sh/beatmaps/1801710/covers/card.jpg",
        cover: "https://assets.ppy.sh/beatmaps/1801710/covers/cover.jpg",
        list: "https://assets.ppy.sh/beatmaps/1801710/covers/list.jpg",
        slimcover:
          "https://assets.ppy.sh/beatmaps/1801710/covers/slimcover.jpg",
      },

      beatmaps: [
        {
          difficulty_rating: 5,
          id: 3535240,
          name: "pjm",
          url: "https://osu.ppy.sh/beatmapsets/1729824#osu/3535240",
        },
      ],
    },
  },
  {
    featured_map_id: 3535240,
    beatmapset: {
      id: 1801710,
      status: BeatmapType.Graveyard,
      creator: "Fursum",
      names: {
        artist: "NCT",
        artist_unicode: "NCT",
        title: "Before I Go (Blooom remix)",
        title_unicode: "Before I Go (Blooom remix)",
      },
      covers: {
        card: "https://assets.ppy.sh/beatmaps/1801710/covers/card.jpg",
        cover: "https://assets.ppy.sh/beatmaps/1801710/covers/cover.jpg",
        list: "https://assets.ppy.sh/beatmaps/1801710/covers/list.jpg",
        slimcover:
          "https://assets.ppy.sh/beatmaps/1801710/covers/slimcover.jpg",
      },

      beatmaps: [
        {
          difficulty_rating: 5,
          id: 3535240,
          name: "pjm",
          url: "https://osu.ppy.sh/beatmapsets/1729824#osu/3535240",
        },
      ],
    },
  },
];

export const DUMMY_USER: UserFullResponse = {
  id: 12345,
  user_name: "Test username",
  profile_picture: "https://a.ppy.sh/4865030?1650115534.jpeg",
  flag: { code: "TR", name: "Türkiye" },
  groups: [
    {
      colour: "red",
      has_listing: true,
      has_playmodes: false,
      id: 1,
      identifier: "NAT",
      is_probationary: false,
      name: "Nomination Assesment Team",
      playmodes: [],
      short_name: "NAT",
    },
  ],
  bio: "Test description",
  featured_maps: exampleMapList,
  graveyard_count: 1,
  loved_count: 0,
  //pending_count: 1,
  ranked_count: 0,
  guest_count: 2,
  nominated_count: 1,
  osu_data_modified_at: Date.now(),
  profile_data_modified_at: Date.now(),

  /*
  influences: [
    {
      profileData: {
        avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
        id: 12345,
        username: "Fursum",
        flag: { code: "TR", name: "Türkiye" },
      },
      type: InfluenceTypeEnum.Respect,
      strength: 2,
      description: "",
      lastUpdated: Date.now(),
      maps: exampleMapList,
    },
    {
      profileData: {
        avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
        id: 1,
        username: "Edisberkserbest",
        groups: [
          {
            colour: "red",
            has_listing: false,
            has_playmodes: false,
            id: 1234,
            identifier: "Nomination Assesment Team",
            is_probationary: false,
            name: "Nomination Assesment Team",
            playmodes: [],
            short_name: "NAT",
          },
        ],
        flag: { code: "TR", name: "Türkiye" },
      },
      type: InfluenceTypeEnum.Respect,
      strength: 1,
      description: "",
      lastUpdated: Date.now(),
      maps: exampleMapList,
    },
    {
      profileData: {
        avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
        id: 2,
        username: "MMMMMMMMMMMMM",
        groups: [
          {
            colour: "red",
            has_listing: false,
            has_playmodes: false,
            id: 1234,
            identifier: "Nomination Assesment Team",
            is_probationary: false,
            name: "Nomination Assesment Team",
            playmodes: [],
            short_name: "NAT",
          },
        ],
        flag: { code: "TR", name: "Türkiye" },
      },
      type: InfluenceTypeEnum.Fascination,
      strength: 3,
      description: "",
      lastUpdated: Date.now(),
      maps: exampleMapList,
    },
    {
      profileData: {
        avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
        id: 3,
        username: "WWWWWWWWWWWWW",
        groups: [
          {
            colour: "purple",
            has_listing: false,
            has_playmodes: false,
            id: 1234,
            identifier: "Nomination Assesment Team",
            is_probationary: false,
            name: "Nomitator",
            playmodes: [],
            short_name: "BN",
          },
        ],
        flag: { code: "TR", name: "Türkiye" },
      },
      type: InfluenceTypeEnum.Implementation,
      strength: 3,
      description: "",
      lastUpdated: Date.now(),
      maps: exampleMapList,
    },
    {
      profileData: {
        avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
        id: 4,
        username: "Fursum",
        flag: { code: "TR", name: "Türkiye" },
      },
      type: InfluenceTypeEnum.Respect,
      strength: 2,
      description: "",
      lastUpdated: Date.now(),
      maps: exampleMapList,
    },
    {
      profileData: {
        avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
        id: 5,
        username: "Fursum",
        flag: { code: "TR", name: "Türkiye" },
      },
      type: InfluenceTypeEnum.Respect,
      strength: 2,
      description: "",
      lastUpdated: Date.now(),
      maps: exampleMapList,
    },
  ],
  mentions: [
    {
      id: 12345,
      username: "Fursum",
      avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
      flag: { code: "TR", name: "Türkiye" },
    },
    {
      id: 123456,
      username: "Fursum",
      avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
      flag: { code: "TR", name: "Türkiye" },
    },
    {
      id: 1234567,
      username: "Fursum",
      avatarUrl: "https://a.ppy.sh/4865030?1650115534.jpeg",
      flag: { code: "TR", name: "Türkiye" },
    },
  ],
  */
};
