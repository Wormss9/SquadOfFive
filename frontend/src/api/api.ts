import axios from "axios";
import { Login, Player, Room, Token, User } from "./types";

const token = document.cookie.split("=")[1];

const baseURL =
  window.location.protocol === "https:"
    ? window.location.origin
    : `http://localhost:7878/`;

const client = axios.create({
  baseURL,
  headers: {
    Authorization: `Bearer ${token}`,
  },
});

export async function login(login: Login) {
  const { data } = await client.get(
    `/api/signing/user?name=${login.name}&password=${login.password}`
  );
  return data as Token;
}

// export async function login_steam(login: Login) {
//   const { data } = await client.get(
//     `/signing/steam?name=${login.name}&password=${login.password}`
//   );
//   return data as Token;
// }

export async function register(login: Login) {
  const { data } = await client.put(`/api/signing/user`, login);
  return data;
}

export async function get_user() {
  const { data } = await client.get(`/api/user`);
  return data as User;
}

export async function get_other_user(id: number) {
  const { data } = await client.get(`/api/user/${id}`);
  return data as User;
}

export async function get_joined_rooms() {
  const { data } = await client.get(`/api/rooms/joined`);
  return data as Room[];
}

export async function get_owned_rooms() {
  const { data } = await client.get(`/api/rooms/owned`);
  return data as Room[];
}

export async function get_room_players(ulid: string) {
  const { data } = await client.get(`/api/room/${ulid}/players`);
  return data as Player[];
}
export function join_game(ulid: string) {
  const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
  const port = window.location.protocol === "https:" ? "" : ":7878";
  const url = `${protocol}//${window.location.hostname}${port}/api/game/${ulid}?token=${token}`;
  const socket = new WebSocket(url);
  return socket;
}

export async function join_room(ulid: string) {
  await client.patch(`/api/room/${ulid}`);
}
