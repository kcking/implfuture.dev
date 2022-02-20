import { useEffect } from "react";
import { run } from "../crates/bevy-example/pkg";

export const BevyExample = () => {
  useEffect(() => {
    run();
  }, []);
  return <></>;
};
