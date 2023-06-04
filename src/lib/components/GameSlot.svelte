<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { join, appDataDir } from "@tauri-apps/api/path";
	import { convertFileSrc } from "@tauri-apps/api/tauri";

	export let game: Game;
	let enableTitleEditing = false;

	$: console.log("Refresh", game);

	enum ImageType {
		Icon = 1,
		Background,
	}

	async function imageToArray(img: File): Promise<number[]> {
		const buffer = await img.arrayBuffer();
		const imageArray = Array.from(new Uint8Array(buffer));
		return imageArray;
	}

	async function handleIconUpload(e: Event) {
		const input = e.target as HTMLInputElement;
		const imageArray = await imageToArray(input.files[0]);
		await invoke("save_image", {
			image: imageArray,
			id: game.id,
			imageType: ImageType.Icon,
		});

		const appDataDirPath = await appDataDir();
		const filePath = await join(appDataDirPath, "icons", `${game.id}.png`);
		const assetUrl = convertFileSrc(filePath);

		game = { ...game, icon: assetUrl };
	}

	async function handleBackgroundUpload(e: Event) {
		const input = e.target as HTMLInputElement;
		const imageArray = await imageToArray(input.files[0]);
		console.log(imageArray);
		await invoke("save_image", {
			image: imageArray,
			id: game.id,
			imageType: ImageType.Background,
		});

		const appDataDirPath = await appDataDir();
		const filePath = await join(
			appDataDirPath,
			"backgrounds",
			`${game.id}.png`
		);
		const assetUrl = convertFileSrc(filePath);

		game = { ...game, background: assetUrl };
	}
</script>

<div class="bg-slate-600 px-5 py-4 flex gap-4 justify-center items-top">
	<div class="flex flex-col justify-between h-full">
		<label class=" relative cursor-pointer group">
			<img
				src={game.icon}
				alt="{game.title}-icon"
				class=" w-28 object-contain group-hover:opacity-20"
			/>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				class="hidden group-hover:block absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-12"
			>
				<path
					d="M4 19H20V12H22V20C22 20.5523 21.5523 21 21 21H3C2.44772 21 2 20.5523 2 20V12H4V19ZM14 9V15H10V9H5L12 2L19 9H14Z"
					fill="rgba(255,255,255,1)"
				/>
			</svg>
			<input
				type="file"
				class="absolute top-0 left-0 w-full h-full hidden"
				on:change={handleIconUpload}
			/>
		</label>
	</div>
	<div class="w-full">
		<div class="flex items-center">
			<button
				class="cursor-pointer"
				on:click={() => (enableTitleEditing = true)}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					class="w-8 fill-white hover:fill-gray-400"
					><path
						d="M7.24264 17.9964H3V13.7538L14.435 2.31877C14.8256 1.92825 15.4587 1.92825 15.8492 2.31877L18.6777 5.1472C19.0682 5.53772 19.0682 6.17089 18.6777 6.56141L7.24264 17.9964ZM3 19.9964H21V21.9964H3V19.9964Z"
					/></svg
				>
			</button>
			<input
				bind:value={game.title}
				class="text-white text-4xl font-semibold bg-transparent outline-none"
				disabled={!enableTitleEditing}
			/>
		</div>
		<label
			class="relative cursor-pointer flex justify-center items-center flex-col border-2 border-gray-800 border-dashed group h-[220px]"
		>
			{#if game.background.length != 0}
				<img
					src={game.background}
					alt="{game.title}-icon"
					class="object-cover h-full w-full group-hover:brightness-50"
				/>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					class="hidden group-hover:block absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-12"
				>
					<path
						d="M4 19H20V12H22V20C22 20.5523 21.5523 21 21 21H3C2.44772 21 2 20.5523 2 20V12H4V19ZM14 9V15H10V9H5L12 2L19 9H14Z"
						fill="rgba(255,255,255,1)"
					/>
				</svg>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					class="w-12 mt-4 group-hover:opacity-40"
					><path
						d="M16 2L21 7V21.0082C21 21.556 20.5551 22 20.0066 22H3.9934C3.44476 22 3 21.5447 3 21.0082V2.9918C3 2.44405 3.44495 2 3.9934 2H16ZM13 12H16L12 8L8 12H11V16H13V12Z"
						fill="white"
					/></svg
				>
				<span class="text-white text-sm font-light mb-4 group-hover:opacity-40"
					>Upload background</span
				>
			{/if}
			<input
				type="file"
				class="absolute top-0 left-0 w-full h-full hidden"
				on:change={handleBackgroundUpload}
			/>
		</label>
	</div>
</div>
