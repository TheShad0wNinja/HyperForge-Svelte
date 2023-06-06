<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { gamesStore } from "../store";
	import GameSlot from "../components/GameSlot.svelte";

	let loading = false;

	async function handleSubmit() {
		loading = true;
		await invoke("update_data", { games: $gamesStore });
		loading = false;
	}

	function addEntry() {
		gamesStore.update((x) => [
			...x,
			{ id: x.length + 1, title: "", icon: "", background: "", path: "" },
		]);
	}
</script>

<div>
	<h1 class="text-white text-5xl font-bold mb-6 pt-6">Game List</h1>
	{#if loading}
		<h2>Loading</h2>
	{:else}
		<div class="mb-4 flex justify-between">
			<h3 class="text-gray-100 text-2xl">{$gamesStore.length} games saved</h3>
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
		<ul class="flex flex-col gap-2">
			{#each $gamesStore as game (game.id)}
				<GameSlot {game} />
			{/each}
		</ul>
		<button class="p-5 bg-red-600 text-white" on:click={handleSubmit}>
			Save
		</button>
	{/if}
</div>
