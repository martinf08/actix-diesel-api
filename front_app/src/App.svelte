<!-- App.svelte -->
<script>
    import { Router, Link, Route } from "svelte-routing";
    import Home from "./routes/Home.svelte";
    import Login from "./routes/Login.svelte";
    import Logout from "./routes/Logout.svelte";

    let home
    export let url = "";
</script>

<Router url="{url}">
    <nav>
        <Link to="/" on:click={() => home.greetings()}>Home</Link>
        <Link to="login">Login</Link>
        <Link to="logout">Logout</Link>
    </nav>
    <div>
        <Route path="/"><Home /></Route>
        <Route path="/login"><Login bind:this={home} /></Route>
        <Route path="/logout"><Logout /></Route>
    </div>
</Router>