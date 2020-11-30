<script lang="typescript">
  import "./Tailwind.svelte";
  import { onMount } from "svelte";
  import init, { is_definite_prime, is_probably_prime } from "./wasm/is_prime";

  onMount(async () => {
    await init();
  });

  const TRIAL_DIVISION_THRESHOLD = BigInt(2 ** 60);
  let input: string;
  let resultDisplay: string = "Type a number to check";
  let timeTaken: number = 0;

  const onClick = () => {
    resultDisplay = "";
    const numbersRegex = /^[0-9]+$/;
    const parsedInput = input.replaceAll(",", "");

    if (!parsedInput || !parsedInput.match(numbersRegex)) {
      alert("Please enter numbers only (commas are allowed)");
      return;
    }

    const bigintInput = BigInt(parsedInput);
    if (bigintInput < TRIAL_DIVISION_THRESHOLD) {
      const t1 = new Date().valueOf();
      const result = is_definite_prime(bigintInput);
      const t2 = new Date().valueOf();
      timeTaken = (t2 - t1) / 1000;
      resultDisplay = result[0]
        ? "✅ prime"
        : "❌ not prime " +
          (bigintInput <= BigInt(1)
            ? "(by definition)"
            : `(divisible by ${result[1].toLocaleString()})`);
    } else {
      const t1 = new Date().valueOf();
      console.log(parsedInput);
      const isProbablyPrime: boolean = is_probably_prime(parsedInput, 30);
      const t2 = new Date().valueOf();
      timeTaken = (t2 - t1) / 1000;
      resultDisplay = isProbablyPrime ? "✅ probably prime" : "❌ not prime";
    }
  };
</script>

<main
  class="flex flex-col justify-center items-center min-h-screen py-10 px-3 text-xl text-gray-100 bg-gray-900">
  <h1
    class="text-2xl text-center text-purple-400 uppercase tracking-widest font-semibold">
    Arbitrary Precision
    <br />
    Primality Test
  </h1>
  <textarea
    class="bg-gray-800 border-2 border-gray-600 rounded-md p-2 m-4 font-mono focus:outline-none focus:border-purple-600 w-full max-w-md"
    type="textarea"
    bind:value={input} />
  <button
    class="py-1 px-3 bg-purple-700 hover:bg-purple-600 rounded font-semibold focus:outline-none focus:shadow-outline"
    on:click={onClick}>
    Check
  </button>
  <div class="overflow-auto my-2">{resultDisplay}</div>
  <div class="fixed bottom-0 right-0 m-3 text-base text-gray-400">
    Time taken:
    {timeTaken.toFixed(2)}s
  </div>
</main>
