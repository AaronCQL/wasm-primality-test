<script lang="typescript">
  import init, { is_definite_prime } from "./wasm/is_prime";

  let input: string;
  let answer: string = "hello";

  const onClick = async () => {
    if (!input) {
      return;
    }
    answer = "";
    await init();
    answer = is_definite_prime(BigInt(input)) + "";
  };
</script>

<div class="App">
  <input type="text" bind:value={input} />
  <button on:click={onClick}>click</button>
  {answer}
</div>
