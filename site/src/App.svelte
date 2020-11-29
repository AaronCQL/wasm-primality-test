<script lang="typescript">
  import "./Tailwind.svelte";
  import { onMount } from "svelte";
  import init, { is_definite_prime } from "./wasm/is_prime";

  onMount(async () => {
    await init();
  });

  let input: string;
  let isPrime: boolean = false;
  let divisor: string = "0";

  const onClick = () => {
    if (!input) {
      return;
    }
    const result = is_definite_prime(BigInt(input));
    isPrime = result[0];
    divisor = result[1].toString();
  };
</script>

<div
  class="flex flex-col justify-center items-center min-h-screen p-10 text-xl">
  <textarea
    class="border border-gray-500 rounded-md p-2 font-mono focus:outline-none focus:border-blue-500 w-screen max-w-md"
    type="textarea"
    bind:value={input} />
  <button
    class="px-2 mt-2 bg-blue-600 text-white rounded-md font-semibold focus:outline-none"
    on:click={onClick}>
    Check
  </button>
  <div class="">{isPrime} {isPrime ? '' : divisor.toString()}</div>
</div>
