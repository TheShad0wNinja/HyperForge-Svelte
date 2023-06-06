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
		await invoke("open_app", { path: game.path });
	}
</script>

{#if !loading}
	<button on:click={openApp}>
		<div
			class="w-[320px] bg-cover bg-fixed bg-no-repeat bg-left h-full grid justify-center items-center relative group"
			style="background-image: url({backgroundUrl});"
		>
			<img src={iconUrl} alt="{game.id}-icon" class="w-36 z-10" />
			<div
				class="h-full w-full absolute bg-black opacity-60 group-hover:opacity-20"
			/>
		</div>
	</button>
{/if}
