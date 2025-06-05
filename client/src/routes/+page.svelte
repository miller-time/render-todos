<script lang="ts">
  type Todo = { text: string; completed: boolean };
  const promise: Promise<Todo[]> = fetch(
    "https://todos-api-u0so.onrender.com/todos",
  ).then((response) => response.json());
</script>

<h1>To Do</h1>
{#await promise then todos}
  <ul>
    {#each todos as todo}
      <li>
        {#if todo.completed}
          <s>{todo.text}</s>
        {:else}
          <span>{todo.text}</span>
        {/if}
      </li>
    {/each}
  </ul>
{/await}
