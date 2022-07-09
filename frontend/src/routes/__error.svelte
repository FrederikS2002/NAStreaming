<script context="module" lang="ts">
	export function load({ error, status }: any) {
		return {
			props: {
				status: status,
				errormsg: error.message
			}
		};
	}
</script>

<script lang="ts">
	export let status: string;
	export let errormsg: string;
</script>

<h1>{status + ' ' + errormsg}</h1>
