<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import Counter from "./lib/Counter.svelte";
  import { invoke } from "@tauri-apps/api";

  let allowedLengths = [
    {value: 11, text: "Small - 11"},
    {value: 15, text: "Medium - 15"},
    {value: 20, text: "Large (Recommended) - 20"}
  ];

  let pashwordPromise;
  let website;
  let username;
  let key;
  let length=20;

  function generate() {
    setTimeout(() => {
      pashwordPromise = invoke("generate_pashword", {
      toHash:
        '{"website":"'+website+'","username":"'+username+'","password":"'+key+'"}',
      pashwordLength: length,
      website: website,
      username: username,
    });
    }, 0)
  }
</script>

<main>
  <div>
    <h2 style="color: crimson;">Pashword-tauri</h2>
  </div>
  <div>
    <form on:submit|preventDefault={generate}>
      Website:
      <input bind:value={website} placeholder="eg: reddit.com"><br><br>
      Username:
      <input bind:value={username} placeholder="eg: random_user1"><br><br>
      Key:
      <input bind:value={key} placeholder="eg: secret_key1!" type="password"><br><br>

      Pashword length:
      <select bind:value={length} title="Pashword length">
        {#each allowedLengths as currLength}
          <option value={currLength.value}>{currLength.text}</option>
        {/each}
      </select>
      <br><br>
      <button disabled={!website||!username||!key||!length} type=submit>
        Generate Pashword
      </button>
    </form>
  </div>

  <div>
    <br><br>
    {#if pashwordPromise}
    {#await pashwordPromise}
      <br><br>
      <p style="color: yellow;">Generating pashword...</p>
    {:then pashword}
      <p>The pashword is</p> <p style="color: greenyellow;">{pashword}</p>
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
    {:else}
      <br><br><br><br>
    {/if}
  </div>
</main>
