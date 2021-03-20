<script>
let name, password, status= '';
async function validate() {
        fetch('http://api.localhost/auth/login', {
            mode: 'cors',
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: JSON.stringify({
                name: name,
                password: password
            })
        })
        .then(response => {
            console.log(response)
            status = response.status
        })
    }
</script>

<form on:submit|preventDefault={validate}>
    <div>
        <label for="name">Username</label>
        <div>
            <input id=name bind:value={name}>
        </div>
    </div>
    <div>
        <label for="password">Password</label>
        <div>
            <input id=password bind:value={password}>
        </div>
    </div>
    <button type="submit">Login</button>
</form>

<p>Response status code : {status}</p>