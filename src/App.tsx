import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(JSON.parse(await invoke("get_dir_stuff", { path: name() })));
    console.log(JSON.stringify(greetMsg(), null, 4))
    // play(`${name()}/${greetMsg()[0]}.mp3`)
  }

  const play = (path: string) => {
    var audio = new Audio(path);
    audio.play();
  }

  return (
    <main class="container">
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{JSON.stringify(greetMsg())}</p>
    </main>
  );
}

export default App;
