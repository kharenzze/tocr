<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let imageUrl = undefined;
  let text = "";
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
    text = await invoke("process_image", { url });
  };
</script>

<main class="container">
  <h1>Welcome to TOCR!</h1>

  <input type="file" on:change={inputHandler} />

  <div class="row container">
    <div class="col half">
      <p>Hi!</p>
      <img class="preview" src={imageUrl} alt="preview" />
    </div>
    <div class="col half">
      <p>Hi!</p>
      <p>{text}</p>
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
