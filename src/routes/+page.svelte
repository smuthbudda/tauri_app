<script lang="ts">
</script>

<main class="container">
  <section class="card_container">
    <a href="/servers" class="card">Server</a>
    <a href="/welcome" class="card">Greet</a>
  </section>

  <p>
    Lets build the most random app with
    <a href="https://macroquad.rs/">macroquad</a>
    <a target="_blank" href="https://tauri.app">Tauri</a>
    <a target="_blank" href="https://kit.svelte.dev">and Svelte</a>
  </p>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;

    input,
    button {
      border-radius: 8px;
      border: 1px solid transparent;
      padding: 0.6em 1.2em;
      font-size: 1em;
      font-weight: 500;
      font-family: inherit;
      color: #0f0f0f;
      background-color: #ffffff;
      transition: border-color 0.25s;
      box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
      cursor: pointer;
    }

    button:hover {
      border-color: #396cd8;
    }
    button:active {
      border-color: #396cd8;
      background-color: #e8e8e8;
    }

    input,
    button {
      outline: none;
    }

    .container {
      margin: 0;
      padding-top: 10vh;
      display: flex;
      flex-direction: column;
      justify-content: center;
      text-align: center;
    }
  }

  .card_container {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }

  .card {
    margin: 1em;
    padding: 1em;
    border: 1px solid #ccc;
    border-radius: 1em;
    background-color: #f6f6f6;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    max-width: 100px;
    align-items: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }
  }
</style>
