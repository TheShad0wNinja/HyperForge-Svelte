<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import SideNav from "./lib/components/SideNav.svelte";
	import Homepage from "./lib/views/Homepage.svelte";
	import OptionsPage from "./lib/views/OptionsPage.svelte";
	import { register } from "@tauri-apps/api/globalShortcut";
	import { appWindow } from "@tauri-apps/api/window";
	import { onDestroy, onMount } from "svelte";

	register("Alt+Shift+G", () => {
		invoke("open_window");
	});

	let unlisten: Function;
	onMount(async () => {
		unlisten = await appWindow.onFocusChanged(({ payload: focused }) => {
			if (!focused) appWindow.hide();
		});
	});

	onDestroy(() => {
		unlisten();
	});

	const PageViews = {
		Homepage: 0,
		Options: 1,
	};

	let page = PageViews.Homepage;

	function toggleSettings() {
		if (page == PageViews.Options) page = PageViews.Homepage;
		else page = PageViews.Options;
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
