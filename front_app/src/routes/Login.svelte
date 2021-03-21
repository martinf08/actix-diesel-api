<script lang="ts">
    import {onMount} from 'svelte'
    import Login from '../components/Login.svelte';

    let data: string[] = []
    onMount(async () => {
        data = await fetch('http://api.localhost/api/?list=10').then(x => x.json())
    })
</script>

<main>
    <table>
        <tr>
            <th>Username</th>
            <th>Password</th>
        </tr>
        {#each data as user}
            <tr>
                <td>{user.name}</td>
                <td>{user.password}</td>
            </tr>
        {/each}
    </table>

    <Login/>
</main>

<style>
    td,
    th {
        border: 1px solid rgb(190, 190, 190);
        padding: 10px;
    }

    td {
        text-align: center;
    }

    tr:nth-child(even) {
        background-color: #eee;
    }

    table {
        border-collapse: collapse;
        border: 2px solid rgb(200, 200, 200);
        letter-spacing: 1px;
        font-family: sans-serif;
        font-size: .8rem;
        margin-bottom: 20px;
    }

    main {
        letter-spacing: 1px;
        font-family: sans-serif;
        font-size: 1rem;
    }
</style>