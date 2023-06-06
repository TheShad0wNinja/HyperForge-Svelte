<script lang="ts">
	import { gamesStore } from "../store";
	import SliderBlock from "./SliderBlock.svelte";
</script>

<div class="h-full grid items-center">
	<div
		class="h-[720px] overflow-x-scroll overflow-y-hidden whitespace-nowrap no-scrollbar flex gap-2"
	>
		{#each $gamesStore as game (game.id)}
			<SliderBlock {game} />
		{/each}
	</div>
</div>

<style>
	.no-scrollbar::-webkit-scrollbar {
		display: none;
	}
</style>
