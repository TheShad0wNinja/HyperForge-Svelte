<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import SideNav from "./lib/components/SideNav.svelte";
	import Homepage from "./lib/views/Homepage.svelte";
	import OptionsPage from "./lib/views/OptionsPage.svelte";
	import { register, isRegistered } from "@tauri-apps/api/globalShortcut";
	import { onMount } from "svelte";

	onMount(async () => {
		const commandIsRegister = await isRegistered("Alt+Shift+G");
		if (!commandIsRegister)
			await register("Alt+Shift+G", () => {
				invoke("open_window");
			});
	});

	const PageViews = {
		Homepage: 0,
		Options: 1,
	};

	let page = PageViews.Homepage;

	function toggleSettings() {
		page = page === PageViews.Options ? PageViews.Homepage : PageViews.Options;
	}
</script>

<div class="container mx-auto h-screen select-none overflow-hidden">
	<SideNav handleClick={toggleSettings} />
	{#if page == PageViews.Homepage}
		<Homepage />
	{:else if page == PageViews.Options}
		<OptionsPage />
	{/if}
</div>

<style>
</style>
