<script>
  import '../app.svelte'
</script>

<nav>
  <a href="/">Home</a>
  <a href="/about">About</a>
  <a href="/settings">Settings</a>
</nav>

<main class="container">
  <slot />
</main>
