import { writable } from "svelte/store";
export const gamesStore = writable<Game[]>([]);
