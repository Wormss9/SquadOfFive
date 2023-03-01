import { Card } from "./types";

export enum WsType {
  Joined = "joined",
  Online = "online",
  Cards = "cards",
}

type IntMessage = {
  type: WsType.Joined;
  message: number;
};

type IntArrMessage = {
  type: WsType.Online;
  message: number[];
};

type CardMessage = {
  type: WsType.Cards;
  message: Card[];
};

export type WsMessage = IntMessage | IntArrMessage | CardMessage;
