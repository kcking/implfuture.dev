import { useEffect } from "react";
import { run } from "../crates/bevy-example/pkg";

export const BevyExample = () => {
  useEffect(() => {
    try {
      run();
    } catch (e) {
      console.error(e);
    }
  }, []);
  return <></>;
};

export default BevyExample;
