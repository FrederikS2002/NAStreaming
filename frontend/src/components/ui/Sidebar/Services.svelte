<script lang="ts">
	import SearchIcon from './svgs/search.svelte';
	import UserIcon from './svgs/user.svelte';
	import SettingsIcon from './svgs/settings.svelte';
	import IconIcon from './svgs/icon.svelte';
	import HomeIcon from './svgs/home.svelte';
	import DiscoverIcon from './svgs/discover.svelte';
	import PlayIcon from './svgs/play.svelte';
	import CategoriesIcon from './svgs/categories.svelte';
	import ErrorIcon from './svgs/error.svelte';
	import AddIcon from './svgs/add.svelte';
	import HeartIcon from './svgs/heart.svelte';
	import Icon from './Icon.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	export let loggedIn = true;
	$: routeId = $page.routeId;
	$: console.log(routeId);
	//restriction: n for no login, l for login, o for open, a for admin //TODO: add admin restriction
	let json = {
		icon: [
			{ name: 'Error', color: [255, 255, 255], routeid: 'error', redirect: '', restriction: 'e' },
			{
				name: 'Add',
				color: [20, 225, 115],
				routeid: 'add',
				redirect: '/add',
				restriction: 'la'
			}
		],
		redirects: [
			{ name: 'Home', color: [249, 230, 80], routeid: '', redirect: '/', restriction: 'l' },
			{
				name: 'Search',
				color: [20, 225, 115],
				routeid: 'search/[...id]',
				redirect: '/search',
				restriction: 'l'
			},
			{
				name: 'Discover',
				color: [100, 220, 250],
				routeid: 'discover',
				redirect: '/discover',
				restriction: 'l'
			},
			{
				name: 'Categories',
				color: [255, 255, 255],
				routeid: 'categories',
				redirect: '/categories',
				restriction: 'l'
			},
			{
				name: 'Info',
				color: [255, 255, 255],
				routeid: 'info/[id]',
				redirect: null,
				restriction: 'lo'
			},
			{
				name: 'Watchlist',
				color: [240, 80, 80],
				routeid: 'watchlist',
				redirect: '/watchlist',
				restriction: 'l'
			}
		],
		user: [
			{ name: 'User', color: [255, 255, 255], routeid: 'user', redirect: '', restriction: 'n' },
			{
				name: 'Home',
				color: [255, 255, 255],
				routeid: 'settings',
				redirect: '/settings',
				restriction: 'l'
			}
		]
	};

	$: filtering = (obj: any) => {
		return (
			((obj.restriction.includes('l') && loggedIn) || !obj.restriction.includes('l')) &&
			((obj.restriction.includes('o') && routeId == obj.routeid) ||
				!obj.restriction.includes('o')) &&
			((obj.restriction.includes('n') && !loggedIn) || !obj.restriction.includes('n')) &&
			((obj.restriction.includes('e') && $page.error != null) || !obj.restriction.includes('e'))
		);
	};
</script>

<div class="sidebar_container">
	<div class="icon">
		<Icon color={[0, 255, 255]}><IconIcon /></Icon>
		{#each json.icon as obj}
			{#if filtering(obj)}
				<Icon
					active={(obj.routeid == 'error' && $page.error != null) || obj.routeid == routeId}
					color={obj.color}
					on:click={(_) => (obj.redirect ? goto(obj.redirect) : _)}
				>
					{#if obj.routeid == 'error'}
						<ErrorIcon />
					{:else if obj.routeid == 'add'}
						<AddIcon />
					{/if}
				</Icon>
			{/if}
		{/each}
	</div>
	<div class="redirects">
		{#each json.redirects as obj}
			{#if filtering(obj)}
				<Icon
					active={obj.routeid == routeId && $page.error == null}
					color={obj.color}
					on:click={(_) => (obj.redirect ? goto(obj.redirect) : _)}
				>
					{#if obj.routeid == ''}
						<HomeIcon />
					{:else if obj.routeid == 'search/[...id]'}
						<SearchIcon />
					{:else if obj.routeid == 'discover'}
						<DiscoverIcon />
					{:else if obj.routeid == 'categories'}
						<CategoriesIcon />
					{:else if obj.routeid == 'info/[id]'}
						<PlayIcon />
					{:else if obj.routeid == 'watchlist'}
						<HeartIcon />
					{/if}
				</Icon>
			{/if}
		{/each}
	</div>
	<div class="user">
		{#each json.user as obj}
			{#if filtering(obj)}
				<Icon
					active={(obj.routeid == routeId || !loggedIn) && $page.error == null}
					color={obj.color}
					on:click={(_) => (obj.redirect ? goto(obj.redirect) : _)}
				>
					{#if obj.routeid == 'user'}
						<UserIcon />
					{:else if obj.routeid == 'settings'}
						<SettingsIcon />
					{/if}
				</Icon>
			{/if}
		{/each}
	</div>
</div>

<style lang="scss">
	@import 'services.scss';
</style>
