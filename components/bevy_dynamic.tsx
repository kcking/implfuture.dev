import dynamic from "next/dynamic";

export const BevyExample = dynamic(() => import("./bevy"), { ssr: false });
