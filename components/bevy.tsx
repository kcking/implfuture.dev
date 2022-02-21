import { useEffect } from "react";
import { run } from "../crates/bevy-example/pkg";

const bevyGlobal = {bevy: null};

export const BevyExample = (props: {width: number; height: number}) => {
  useEffect(() => {
    try {
      run(props.width, props.height);
    } catch (e) {
      console.error(e);
    }
  }, []);
  return <></>;
};

export default BevyExample;
