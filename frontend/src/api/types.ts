export type Login = {
  name: string;
  password: string;
};
export type Room = {
  ulid: string;
  host: number;
  play: Card[];
  turn: number;
};

export type Card = {
  color: string;
  value: string;
};

export type User = {
  id: number;
  nick: string;
  avatar: string;
};

export type Player = {
  id: number;
  game_user: number;
  room: string;
  points: number;
  turn: number;
};

export type Token = {
  Authorization: string;
};

export type Gamer = User & Player;

export type Rooms = Gamer[][];
