<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import type { ServerDetails } from "../../types/types";
  import { onMount } from "svelte";


  let address = $state("");
  let port = $state("");
  let name = $state("");

  let serverMsg = $state("");
  let updResponses: string[] = $state([]);
  let servers: ServerDetails[] = $state([]);

  onMount(async () => {
    await loadServers();

    // Listen for incoming UDP messages
    listen("udp-message", (event) => {
      if (typeof event.payload === "string") {
        updResponses = [...updResponses, event.payload];
      }
    });
  });

  // Fetch servers from the backend
  async function loadServers() {
    try {
      serverMsg = await invoke<string>("get_servers");
      servers = JSON.parse(serverMsg);
    } catch (error) {
      console.error("Failed to load servers:", error);
    }
  }

  listen("udp-message", (event) => {
    console.log("UDP message received:", event.payload);
    if (typeof event.payload === "string") {
      updResponses.push(event.payload);
    } else if (event.payload && typeof event.payload === "object") {
      updResponses.push(JSON.stringify(event.payload));
    }
  });

  async function ServerResponse(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    serverMsg = await invoke("create_server", { address, port, name });
    let serverDetails: ServerDetails[] = JSON.parse(serverMsg);
    servers = serverDetails;
  }
</script>

<section class="container">
  {#if servers.length < 3}
  <form class="row" onsubmit={ServerResponse}>
    <input id="port-input" placeholder="port" bind:value={port} />
    <input id="namer-input" placeholder="Server Name" bind:value={name} />
    <button type="submit">Create Server</button>
  </form>
  {/if}
  {#each updResponses as response}
    <p>{response}</p>
  {/each}
</section>

<section class="container">
  <table class="styled-table">
    <thead>
      <tr>
        <th>Name</th>
        <th>Address</th>
        <th>Port</th>
        <th>Action</th>
      </tr>
    </thead>
    <tbody>
      {#each servers as server}
        <tr>
          <td>{server.name}</td>
          <td>{server.address}</td>
          <td>{server.port}</td>
          <td><button class="delete-btn">X</button></td>
        </tr>
      {/each}
    </tbody>
  </table>
</section>

<style>
  

  .styled-table {
    width: 100%;
    border-collapse: collapse;
    font-family: Arial, sans-serif;
    margin: 20px 0;
    font-size: 16px;
    text-align: left;
  }

  .styled-table thead tr {
    background-color: #009879;
    color: #ffffff;
    text-align: left;
  }

  .styled-table th,
  .styled-table td {
    padding: 12px 15px;
  }

  .styled-table tbody tr {
    border-bottom: 1px solid #dddddd;
  }

  .styled-table tbody tr:nth-of-type(even) {
    background-color: black;
  }

  .styled-table tbody tr:last-of-type {
    border-bottom: 2px solid #009879;
  }

  .styled-table tbody tr:hover {
    background-color: #f1f1f1;
  }

  .delete-btn {
    background-color: #ff4d4d;
    color: white;
    border: none;
    padding: 5px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
  }

  .delete-btn:hover {
    background-color: #ff1a1a;
  }

  .row {
    display: flex;
    justify-content: center;
    gap: 1em;
  }


  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
