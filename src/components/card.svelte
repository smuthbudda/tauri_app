<script>
    export let title = "Card Title";
    export let content = "This is some card content.";
    export let image = ""; // Optional image URL
  </script>
  
  <div class="card">
    {#if image}
      <img src={image} alt={title} class="card-image" />
    {/if}
    <div class="card-content">
      <h2 class="card-title">{title}</h2>
      <p class="card-text">{content}</p>
    </div>
  </div>
  
  <style>
    .card {
      border: 1px solid #ddd;
      border-radius: 8px;
      overflow: hidden;
      max-width: 300px;
      box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
      font-family: Arial, sans-serif;
    }
  
    .card-image {
      width: 100%;
      height: auto;
    }
  
    .card-content {
      padding: 16px;
    }
  
    .card-title {
      font-size: 1.25rem;
      margin: 0 0 8px 0;
    }
  
    .card-text {
      font-size: 1rem;
      margin: 0;
      color: #555;
    }
  </style>
  