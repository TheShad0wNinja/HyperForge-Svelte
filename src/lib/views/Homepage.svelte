<script lang="ts">
	import Slider from "../components/Slider.svelte";

	import { invoke } from "@tauri-apps/api/tauri";
	import { gamesStore } from "../store";
	import { onMount } from "svelte";

	onMount(async () => {
		const data = (await invoke("load_data").catch((err) =>
			console.log(err)
		)) as Game[];
		gamesStore.set(data);
	});
</script>

<Slider />
