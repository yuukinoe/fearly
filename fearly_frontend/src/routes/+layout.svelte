<script lang="ts">
	import { resolve } from '$app/paths';
	import { logoutUser } from '$lib/client.ts';
	import { SignOut } from 'phosphor-svelte';
	import '../app.css';
	import { pwaInfo } from 'virtual:pwa-info';
	const { data, children } = $props();
	const home = resolve('/');
	const list = resolve(`/list`);
	const handleLogout = async () => {
		await logoutUser();
		window.location.href = '/';
		return;
	};
	let webManifestLink = $derived(pwaInfo ? pwaInfo.webManifest.linkTag : '');
</script>

<svelte:head>
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
	{@html webManifestLink}
</svelte:head>
<div class="relative min-h-screen">
	<div
		class={`border-b-solid ${data.isLoggedIn ? '' : 'fixed'} w-full border-b border-b-[#27272a]`}
	>
		<div class="grid grid-cols-3">
			<div class="sm:hidden"></div>
			<div class="flex w-full items-center justify-center p-7 text-[#f2f2f2] sm:justify-start">
				<span class="font-roboto text-5xl font-bold select-none"><a href={home}>fearly</a></span>
			</div>
			<div
				class="hidden items-center justify-center font-roboto text-xl font-bold text-[#f2f2f2] sm:flex"
			>
				<div class="flex gap-5 select-none">
					<a href={home}>Strona główna</a>
					<a href={list}>Lista</a>
				</div>
			</div>
			<div class="flex w-full items-center justify-end p-7 text-[#f2f2f2]">
				{#if data.isLoggedIn}
					<button
						class="cursor-pointer rounded-lg p-2 px-5 font-roboto hover:opacity-90 sm:bg-white sm:text-black sm:active:scale-99"
						onclick={handleLogout}
					>
						<span class="hidden sm:block"> Wyloguj się </span>
						<div class="rounded border border-solid border-[#27272a] p-2 sm:hidden">
							<SignOut class="h-5 w-5" />
						</div>
					</button>
				{/if}
			</div>
		</div>
	</div>
	<div class="">
		{@render children()}
	</div>
	{#if data.isLoggedIn}
		<div
			class="fixed bottom-0 z-2 flex w-full flex-col items-center justify-center border-t border-solid border-t-[#27272a] bg-[#0d0d0d] sm:hidden"
		>
			<div class="flex gap-5 py-8 font-roboto font-bold text-white select-none">
				<a href={home}>Strona główna</a>
				<a href={list}>Lista</a>
			</div>
		</div>
	{/if}
</div>
{#if data.isLoggedIn}
	<div class="block sm:hidden">
		<div class="flex flex-col items-center justify-center">
			<div class="flex gap-5 py-11 font-roboto font-bold text-white select-none">
				<!-- <a href={home}>Strona główna</a> -->
				<!-- <a href={list}>Lista</a> -->
			</div>
		</div>
	</div>
{/if}
