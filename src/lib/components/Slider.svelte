<script lang="ts">
	import { gamesStore } from "../store";
</script>

<div class="h-full flex items-center">
	<div
		class="h-[720px] overflow-x-scroll overflow-y-hidden whitespace-nowrap no-scrollbar"
	>
		{#each $gamesStore as game (game.id)}
			<div
				class="inline-block mx-2 w-[320px] h-full bg-left relative group bg-fixed bg-cover bg-no-repeat overflow-hidden cursor-pointer"
				style="background-image: url({game.background});"
			>
				<img
					src={game.icon}
					alt="{game.title}-icon"
					class=" w-36 absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-10"
				/>
				<div
					class="h-full w-full absolute opacity-60 bg-black group-hover:opacity-20 transition-opacity"
				/>
			</div>
		{/each}
	</div>
</div>

<style>
	.no-scrollbar::-webkit-scrollbar {
		display: none;
	}
</style>
