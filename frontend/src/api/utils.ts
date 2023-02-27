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
          return { ...player, ...user } as User & Player;
        })
    );
    return [turn, nroom] as [number, (User & Player)[]];
  });
  const playersTurns = await Promise.all(playersTurnsPromises);

  const x = playersTurns.map(([turn, players]) => {
    return players.sort((a, b) => {
      const x = a.turn < turn ? a.turn + 4 : a.turn;
      const y = b.turn < turn ? b.turn + 4 : b.turn;
      return x - y;
    });
  });

  console.log(x);
  return x;
}
