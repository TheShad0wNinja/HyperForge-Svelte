<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { gamesStore } from "../store";
	import GameSlot from "../components/GameSlot.svelte";
	import { scale } from "svelte/transition";
	import { quintInOut } from "svelte/easing";
	import Spinner from "../components/Spinner.svelte";
	import { message } from "@tauri-apps/api/dialog";
	import { flip } from "svelte/animate";

	let loading = false;

	async function handleSubmit() {
		loading = true;
		await invoke("update_data", { games: $gamesStore });
		await message("List saved");
		loading = false;
	}

	function addEntry() {
		gamesStore.update((x) => [
			...x,
			{ id: x.length + 1, title: "", icon: "", background: "", path: "" },
		]);
	}

	function deleteEntry(e: CustomEvent) {
		gamesStore.update((x) => x.filter((y) => y.id !== e.detail.id));
	}
</script>

<div
	in:scale={{ duration: 200, easing: quintInOut, start: 2, delay: 200 }}
	out:scale={{ duration: 200, easing: quintInOut, start: 2.0 }}
	class="flex flex-col gap-4 py-8"
>
	<h1 class="text-white text-5xl font-bold">Game List</h1>
	{#if !loading}
		<div class=" flex justify-between">
			<h3 class="text-gray-100 text-2xl">
				{$gamesStore.length}
				{$gamesStore.length == 1 ? "game" : "games"} saved
			</h3>
			<button on:click={addEntry}>
				<h3
					class="text-gray-100 text-3xl hover:cursor-pointer group hover:text-gray-400 active:scale-95"
				>
					Add new game <svg
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
						aria-hidden="true"
						class="fill-white inline w-8 group-hover:fill-gray-400 group-active:scale-95"
					>
						<path
							clip-rule="evenodd"
							fill-rule="evenodd"
							d="M12 3.75a.75.75 0 01.75.75v6.75h6.75a.75.75 0 010 1.5h-6.75v6.75a.75.75 0 01-1.5 0v-6.75H4.5a.75.75 0 010-1.5h6.75V4.5a.75.75 0 01.75-.75z"
						/>
					</svg>
				</h3>
			</button>
		</div>
		<ul
			class="flex flex-col gap-2 max-h-[70vh] overflow-y-scroll whitespace-nowrap"
		>
			{#each $gamesStore as game (game.id)}
				<div animate:flip={{ duration: 300 }}>
					<GameSlot {game} on:delete={deleteEntry} />
				</div>
			{/each}
		</ul>
		<div class="w-full grid justify-center">
			<button
				class="bg-gray-50 border-2 border-slate-800, rounded-lg font-semibold leading-5 px-10 py-3 text-center shadow hover:bg-gray-300 active:scale-95 focus-visible:shadow-none"
				on:click={handleSubmit}
			>
				Save
			</button>
		</div>
	{:else}
		<Spinner />
	{/if}
</div>
