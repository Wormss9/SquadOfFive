import { Card } from "./types";

export enum WsType {
  Joined = "joined",
  Online = "online",
  Cards = "cards",
  Table = "table",
  Turn = "turn",
  Left = "left",
  CardAmmount = "card_amount",
}

type JoinedMessage = {
  type: WsType.Joined;
  message: number;
};

type OnlineMessage = {
  type: WsType.Online;
  message: number[];
};

type CardMessage = {
  type: WsType.Cards;
  message: Card[];
};

type TableMessage = {
  type: WsType.Table;
  message: Card[];
};

type TurnMessage = {
  type: WsType.Turn;
  message: number;
};

type LeftMessage = {
  type: WsType.Left;
  message: number;
};

type CardAmmountMessage = {
  type: WsType.CardAmmount;
  message: [number, number];
};

export type WsMessage =
  | JoinedMessage
  | OnlineMessage
  | CardMessage
  | TableMessage
  | TurnMessage
  | LeftMessage
  | CardAmmountMessage;
