<script lang="ts">
	import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	export let game: Game;

	let loading = true;

	let iconUrl: string, backgroundUrl: string;

	onMount(async () => {
		iconUrl = game.icon.length > 0 ? convertFileSrc(game.icon) : "";
		backgroundUrl =
			game.background.length > 0 ? convertFileSrc(game.background) : "";
		loading = false;
	});

	async function openApp() {
		if (game.path.length == 0) return;
		await invoke("open_app", { path: game.path });
	}
</script>

{#if !loading}
	<button on:click={openApp} class="flex-[0_0_25%]">
		<div
			class="w-full bg-cover bg-fixed bg-no-repeat bg-left h-full grid justify-center items-center relative group inner-glow"
			style="background-image: url({backgroundUrl});"
		>
			{#if iconUrl.length > 0}
				<img src={iconUrl} alt="{game.id}-icon" class="w-36 z-10" />
			{:else}
				<svg
					fill="none"
					stroke="white"
					stroke-width="1.5"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					aria-hidden="true"
					class="w-16 z-10"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
			{/if}
			<div
				class="h-full w-full absolute bg-black opacity-60 group-hover:opacity-20"
			/>
		</div>
	</button>
{/if}

<style>
	.inner-glow:hover {
		box-shadow: inset 0px 0px 20px 1px rgba(255, 255, 255, 0.9);
	}
</style>
