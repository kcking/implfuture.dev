import { Component, ReactElement } from "react";

type ProjectProps = {
  name: string;
  description: ReactElement;
  image: string;
};

function Project(props: ProjectProps) {
  return (
    <div
      className="group p-2 border-0 border-zinc-700 border-solid w-64 h-64 flex flex-col justify-between select-none "
      tabIndex={0}
      key={props.name}
    >
      <div
        className={
          "w-60 h-60 absolute group-hover:invisible group-focus:invisible"
        }
        style={{
          backgroundImage: `url('${props.image}')`,
          backgroundSize: "80% 80%",
          backgroundPosition: "50% 20%",
          backgroundRepeat: "no-repeat",
        }}
      ></div>
      <p className="invisible group-hover:visible group-focus:visible">
        {props.description}
      </p>
      <div className="text-2xl font-display place-self-center">
        {props.name}
      </div>
    </div>
  );
}

const projects: Array<ProjectProps> = [
  {
    name: "Bevy OpenXR",
    description: (
      <>
        Unity and Unreal Engine are practically the only options when developing
        VR games and experiences. I am actively bringing OpenXR support to the
        Bevy game engine in order to spread the benefits of rust to XR
        development.
      </>
    ),
    image: "/bevy.svg",
  },
  {
    name: "Ovrlay",
    description: (
      <>
        View Discord notifications while {"you're"} in VR. Built with OpenVR
        rust bindings and{" "}
        <a href="https://github.com/iced-rs/iced">
          <code>iced</code>{" "}
        </a>{" "}
        native GUI toolkit. Forked <code>iced_glow</code> to render to an OpenGL
        texture which is then composited into VR using the OpenVR overlay API.{" "}
        <a href="https://store.steampowered.com/app/1384020/Ovrlay__VR_Discord_Notifications/">
          Published on Steam.
        </a>
      </>
    ),
    image: "/ovrlay.svg",
  },
  {
    name: "Bigroom",
    description: (
      <>
        Social video chat for large groups. Built with WebRTC and WebAudio,
        works directly in the browser. Full stack Rust web application using{" "}
        <code>async-tungstenite</code> and <code>yew</code>.
      </>
    ),
    image: "/bigroom.svg",
  },
  {
    name: "Just Fly",
    description: (
      <>
        A mod for the factory simulation game{" "}
        <a href="https://www.satisfactorygame.com/">Satisfactory</a>. Provides
        ergonomic flight controls for an optimal {'"Creative Mode"'}
        experience. Built using Unreal Engine blueprints. Published on{" "}
        <a href="https://ficsit.app/mod/JustFly">ficsit.app</a> and available on{" "}
        <a href="https://github.com/kcking/justfly">GitHub</a>.
      </>
    ),
    image: "justfly.png",
  },
  {
    name: "Clickhouse",
    description: (
      <>
        Rearchitected <a href="https://mux.com">Mux</a>
        {"'s"} video analytics product from Citus to{" "}
        <a href="https://clickhouse.com">ClickHouse</a>. Deployed using
        Kubernetes to be horizontally and vertically scalable with zero
        downtime. Check out the corresponding{" "}
        <a
          href="https://mux.com/blog/from-russia-with-love-how-clickhouse-saved-our-data/"
          rel="noopener noreferrer"
        >
          blog post
        </a>
        .
      </>
    ),
    image: "/clickhouse.svg",
  },
  {
    name: "Krypton",
    description: (
      <>
        <a href="https://krypt.co" rel="noopener noreferrer">
          Phone-based phishing-proof 2FA.
        </a>{" "}
        Built on a trustless infrastructure using end-to-end encryption between
        {"user's"} devices. Technology aquired by Akamai. All client software
        published on <a href="https://github.com/kryptco/">GitHub</a>.
      </>
    ),
    image: "/krypton.svg",
  },
  {
    name: "Ears",
    description: (
      <>
        <a
          href="https://chrome.google.com/webstore/detail/ears-bass-boost-eq-any-au/nfdfiepdkbnoanddpianalelglmfooik"
          rel="noopener noreferrer"
        >
          Chrome extension
        </a>{" "}
        that provides a graphical equalizer, volume, or bass boost for any
        webpage. Used by more than 300,000 audiophiles, hard-of-hearing, and
        transcriptionists world-wide.
      </>
    ),
    image: "/chrome.svg",
  },
];

export default function Projects() {
  return (
    <div className="flex flex-wrap gap-10 justify-center max-w-6xl">
      {projects.map((p) => {
        return Project(p);
      })}
    </div>
  );
}
