<script lang="ts">
  const getDataUrl = async (file) => {
    const fReader = new FileReader();
    fReader.readAsDataURL(file);
    const readPromise = new Promise((resolve, reject) => {
      fReader.onloadend = (evt) => resolve(evt.target.result);
    });
    return readPromise;
  };

  const inputHandler = async (evt) => {
    const file = evt.target.files[0];
    const url = await getDataUrl(file);
    console.log(url);
  };
</script>

<main class="container">
  <h1>Welcome to TOCR!</h1>

  <input type="file" on:change={inputHandler} />

  <div class="row">
    <div class="half">
      <p>Hi!</p>
    </div>
    <div class="half">
      <p>Hi!</p>
    </div>
  </div>
</main>

<style>
  .half {
    flex: 1;
    display: flex;
    justify-content: center;
  }
</style>

