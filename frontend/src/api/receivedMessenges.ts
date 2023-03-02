import { Card } from "./types";

export enum WsType {
  Joined = "joined",
  Online = "online",
  Cards = "cards",
  Table = "table",
  Turn = "turn",
  Left = "left",
  CardAmmount = "card_amount",
  Error = "error",
}

type Joined = {
  type: WsType.Joined;
  message: number;
};

type Online = {
  type: WsType.Online;
  message: number[];
};

type Cards = {
  type: WsType.Cards;
  message: Card[];
};

type Table = {
  type: WsType.Table;
  message: Card[];
};

type Turn = {
  type: WsType.Turn;
  message: number;
};

type Left = {
  type: WsType.Left;
  message: number;
};

type CardAmmount = {
  type: WsType.CardAmmount;
  message: [number, number];
};

type Error = {
  type: WsType.Error;
  message: string;
};

export type WsMessage =
  | Joined
  | Online
  | Cards
  | Table
  | Turn
  | Left
  | CardAmmount
  | Error;
