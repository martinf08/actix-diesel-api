<!-- App.svelte -->
<script>
    import {Router, Link, Route} from "svelte-routing";
    import {onMount} from "svelte";
    import Home from "./routes/Home.svelte";
    import Login from "./routes/Login.svelte";
    import Logout from "./routes/Logout.svelte";
    import CreateUser from "./routes/CreateUser.svelte";

    let userLogged = false;
    let isLogged = async () => userLogged = await fetch(
        "http://api.localhost/auth/is_logged",
        {credentials: 'include'}
    ).then(res => res.json())

    onMount(isLogged)

    let home
    export let url = "";
</script>

<Router url="{url}">
    <nav>
        <Link to="/" on:click={isLogged}>Home</Link>
        <Link to="login" on:click={isLogged}>Login</Link>
        <Link to="logout">Logout</Link>
        {#if userLogged}
            <Link to="user_create">Create User</Link>
        {/if}
    </nav>
    <div>
        <Route path="/"><Home /></Route>
        <Route path="/login"><Login bind:this={home} /></Route>
        <Route path="/logout"><Logout /></Route>
        {#if userLogged}
            <Route path="/user_create"><CreateUser /></Route>
        {/if}
    </div>
</Router>