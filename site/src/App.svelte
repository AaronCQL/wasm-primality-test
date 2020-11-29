<script lang="typescript">
  import "./Tailwind.svelte";
  import { onMount } from "svelte";
  import init, { is_definite_prime } from "./wasm/is_prime";

  onMount(async () => {
    await init();
  });

  let input: string = "121";
  let answer: string = "hello";

  const onClick = async () => {
    if (!input) {
      return;
    }
    answer = is_definite_prime(BigInt(input)) + "";
  };
</script>

<div
  class="flex flex-col justify-center items-center min-h-screen p-10 text-xl">
  <textarea
    class="border border-gray-500 rounded-md p-2 font-mono focus:outline-none focus:border-blue-500"
    type="textarea"
    bind:value={input} />
  <button
    class="p-1 mt-2 bg-blue-600 text-white rounded-md font-semibold focus:outline-none"
    on:click={onClick}>
    CHECK
  </button>
  <div class="">{answer}</div>
</div>
