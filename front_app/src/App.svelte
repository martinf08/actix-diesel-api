<script lang="ts">
    import {onMount} from 'svelte'

    let data: string[] = []
    onMount(async () => {
        data = await fetch('http://api.localhost/api/?list=100').then(x => x.json())
    })
</script>

<main>
    <ul>
        {#each data as user}
            <li>{user.name}</li>
        {/each}
    </ul>

    <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>


<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>