<script lang="ts">
  let imageUrl = undefined;
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
    imageUrl = url;
  };
</script>

<main class="container">
  <h1>Welcome to TOCR!</h1>

  <input type="file" on:change={inputHandler} />

  <div class="row container">
    <div class="col half">
      <p>Hi!</p>
      <img class="preview" src={imageUrl} />
    </div>
    <div class="col half">
      <p>Hi!</p>
    </div>
  </div>
</main>

<style>
  .half {
    flex: 0 0 1;
    display: flex;
    justify-content: center;
    max-width: 50%;
    box-sizing: border-box;
  }

  .preview {
    object-fit: contain;
  }
</style>
