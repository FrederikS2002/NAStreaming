<script lang="ts">
    export let epilist: any[];
    import Episode from './Redirect.svelte';
    import {generateFML} from './index';
    import {url} from '../../urls';
    </script>
<div class="episodes">
    {#each epilist as episode, i}
    <Episode {episode} fml ={generateFML(i, epilist.length)} url = "{url+'/play/'+episode.uuid}" />
    {/each}
</div>