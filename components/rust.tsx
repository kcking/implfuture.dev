import { useState } from "react";
import { rust_string } from "../rust/pkg";

export default function RustString() {
  const [counter, setCounter] = useState(0);
  return (
    <span onClick={() => setCounter((counter) => counter + 1)}>
      {rust_string() + `${counter}`}
    </span>
  );
}
