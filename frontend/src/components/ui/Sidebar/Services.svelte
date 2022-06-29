<script lang="ts">
	import SearchIcon from './svgs/search.svelte';
	import UserIcon from './svgs/user.svelte';
	import SettingsIcon from './svgs/settings.svelte';
	import IconIcon from './svgs/icon.svelte';
	import HomeIcon from './svgs/home.svelte';
	import DiscoverIcon from './svgs/discover.svelte';
	import PlayIcon from './svgs/play.svelte';
	import CategoriesIcon from './svgs/categories.svelte';
	import './services.scss';
	import HeartIcon from './svgs/heart.svelte';
	import Icon from './Icon.svelte';
	import { goto } from '$app/navigation';
	export let loggedIn = true;
	export let info = false;
</script>

<div class="sidebar_container">
	<div class="icon">
		<Icon color={[0, 255, 255]}><IconIcon /></Icon>
	</div>
	<div class="redirects">
		{#if loggedIn}
			<Icon active={true}  color={[249, 230, 80]} on:click={(e) => goto('/')}><HomeIcon /></Icon>
			<Icon color={[20, 225, 115]} on:click={(e) => goto('/search')}><SearchIcon /></Icon>
			<Icon color={[100, 220, 250]} on:click={(e) => goto('/discover')}><DiscoverIcon /></Icon>
			<Icon color={[255, 255, 255]} on:click={(e) => goto('/categories')}><CategoriesIcon /></Icon>
			{#if info}
				<Icon color={[255, 255, 255]} on:click={(e) => goto('/info')}><PlayIcon /></Icon>
			{/if}
			<Icon color={[240, 80, 80]} on:click={(e) => goto('/watchlist')}><HeartIcon /></Icon>
		{/if}
	</div>
	<div class="user">
		{#if !loggedIn}
			<Icon color={[255, 255, 255]}><UserIcon /></Icon>
		{:else}
			<Icon color={[255, 255, 255]} on:click={(e) => goto('/settings')}><SettingsIcon /></Icon>
		{/if}
	</div>
</div>