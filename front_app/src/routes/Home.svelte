<script lang="ts">
    import {onMount} from "svelte"

    let message = '';
    let greetings = async () => message = await fetch(
        "http://api.localhost/auth/is_logged",
        {credentials: 'include'}
    ).then(res => res.json())

    onMount(greetings)
</script>

<p>{message}</p>