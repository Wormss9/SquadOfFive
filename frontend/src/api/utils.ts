import {
  get_joined_rooms,
  get_other_user,
  get_owned_rooms,
  get_room_players,
} from "./api";
import { Player, Room, User } from "./types";

export function form_event_to_object<T>(event: Event): T {
  const iterable = new FormData(event.target as HTMLFormElement).entries();
  return collect(iterable);
}

function collect<T, U>(iter: IterableIterator<[string, T]>) {
  const object: Record<string, T> = {};
  for (const [key, value] of iter) {
    object[key] = value;
  }
  return object as unknown as U;
}

export async function get_rooms_with_users() {
  const joined = await get_joined_rooms();
  const owned = await get_owned_rooms();
  return {
    owned: await rooms_to_rooms_with_users(owned),
    joined: await rooms_to_rooms_with_users(joined),
  };
}

export async function get_room_with_users(ulid: string) {
  const mockRoom = { ulid, turn: 0 };
  const rooms = await rooms_to_rooms_with_users([mockRoom as Room]);
  return rooms[0];
}

async function rooms_to_rooms_with_users(rooms: Room[]) {
  const playersTurnsPromises = rooms.map(async (room) => {
    const turn = room.turn;
    const nroom = await Promise.all(
      (
        await get_room_players(room.ulid)
      )
        .filter((player) => player.game_user)
        .map(async (player) => {
          const user = await get_other_user(player.game_user);
          return { ...user, ...player, userId: user.id } as User &
            Player & { userId: number };
        })
    );
    return [turn, nroom] as [number, (User & Player & { userId: number })[]];
  });
  const playersTurns = await Promise.all(playersTurnsPromises);

  const sorted = playersTurns.map(([turn, players]) => {
    return players.sort((a, b) => {
      const x = a.turn < turn ? a.turn + 4 : a.turn;
      const y = b.turn < turn ? b.turn + 4 : b.turn;
      return x - y;
    });
  });

  return sorted;
}

const value_map = {
  One: "01",
  Two: "02",
  Three: "03",
  Four: "04",
  Five: "05",
  Six: "06",
  Seven: "07",
  Eight: "08",
  Nine: "09",
  Ten: "10",
  Aquamarine: "11",
  Diamond: "12",
};

export function be_to_to_value(value?: string) {
  if (!value) return "01";
  return value_map[value as keyof typeof value_map];
}
export function be_to_to_color(color?: string) {
  if (!color) return "r";
  return color[0].toLowerCase();
}
