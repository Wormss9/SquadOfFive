import axios from "axios";
import { Login, Player, Room, Token, User } from "./types";

const token = document.cookie.split("=")[1];

const path = "localhost:7878/api";

const client = axios.create({
  baseURL: `http://${path}`,
  headers: {
    Authorization: `Bearer ${token}`,
  },
});

export async function login(login: Login) {
  const { data } = await client.get(
    `/signing/user?name=${login.name}&password=${login.password}`
  );
  return data as Token;
}

export async function register(login: Login) {
  const { data } = await client.put(`/signing/user`, login);
  return data;
}

export async function get_user() {
  const { data } = await client.get(`/user`);
  return data as User;
}

export async function get_other_user(id: number) {
  const { data } = await client.get(`/user/${id}`);
  return data as User;
}

export async function get_joined_rooms() {
  const { data } = await client.get(`/rooms/joined`);
  return data as Room[];
}

export async function get_owned_rooms() {
  const { data } = await client.get(`/rooms/owned`);
  return data as Room[];
}

export async function get_room_players(ulid: string) {
  const { data } = await client.get(`/room/${ulid}/players`);
  return data as Player[];
}
export async function join_room(id: string) {
  const socket = new WebSocket(`ws://${path}/game/${id}?token=${token}`);
  return socket;
}
