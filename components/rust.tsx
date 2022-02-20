import { useState } from "react";
import { add_one } from "../rust/pkg";

export const Counter = () => {
  const [counter, setCounter] = useState(0);
  return (
    <span
      className="select-none bg-gray-300/30 p-2 rounded"
      onClick={() => setCounter((counter) => add_one(counter))}
    >
      Counter {counter}
    </span>
  );
};
