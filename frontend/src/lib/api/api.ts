import { PATH } from "./path";
import { form_event_to_object } from "./utils";
import axios from 'axios';

const a = axios.create({
	baseURL: PATH
});

export async function login(event: SubmitEvent) {
	const login: Login = form_event_to_object(event)

	const res = await a.get(`/login/user?name=${login.name}&password=${login.password}`)

	console.log(res)
}
type Login = {
	name: string,
	password: string
};

