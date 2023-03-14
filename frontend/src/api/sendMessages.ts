import { Card } from "./types";

enum SendType {
  Play = "play",
  Skip = "skip",
}

export const playMessage = (cards: Card[]) => ({
  type: SendType.Play,
  message: cards,
});

export const skipMessage = () => ({
  type: SendType.Skip,
  message: "skip",
});
