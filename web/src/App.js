import React, { h, render, Component, useState, useCallback, useEffect } from "react";

function Counter() {
  const [wasm, setWasm] = useState(null);
  const [value, setValue] = useState("");

  useEffect(() => {
    async function fetch() {
      try {
        const w = await import("pipers");
        setWasm(w)
      } catch (err) {
        console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
      }
    }
    fetch()
  }, []);

  const convert = useCallback((x) => {
    if (wasm != null) {
      return wasm.convert(x)
    } else {
      return "loading..."
    }
  }, [wasm])

  const onChange = (event) => {
    setValue(event.target.value)
  }

  return (
    <div>
      <textarea value={value} onInput={onChange} />
      <div>
        {convert(value)}
      </div>
    </div>
  );
}

export default function App() {
  return (
    <div>
      <h1>Hello, World!</h1>
      <Counter />
    </div>
  );
}
