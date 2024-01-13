import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import Notebook from "./Notebook";

export default () => {
  const [text, setText] = useState("");
  const [greeting, setGreeting] = useState("");
  const [cpuUsage, setCpuUsage] = useState(0);

  useEffect(() => {
    const interval = setInterval(async () => {
      setCpuUsage(await invoke("cpu_usage"));
    }, 200);

    return () => clearInterval(interval);
  }, []);

  return (
    <main className="px-4 py-8">
      <div className="text-3xl">Jute!</div>
      <form
        onSubmit={async (event) => {
          event.preventDefault();
          setGreeting(await invoke("greet", { name: text }));
        }}
      >
        <input
          value={text}
          onChange={(event) => setText(event.target.value)}
          className="border border-black p-1"
        />
        <button className="ml-2 border border-black p-1">Submit</button>

        {greeting && <p>Returned: {greeting}</p>}
      </form>

      <p>CPU Usage: {cpuUsage.toFixed(2)}%</p>

      <Notebook />
    </main>
  );
};
