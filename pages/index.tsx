import type { NextPage } from "next";

const Home: NextPage = () => (
  <>
    <div className="w-full font-body text-3xl my-10 flex justify-evenly px-2 flex-wrap">
      <div className="max-w-md">
        <div>
          <strong className="font-extrabold">im·pl</strong>
        </div>
        <div>[im-pʊl]</div>
        <div className="">
          <em>v</em>. to bring into existence something previously impossible,
          usually by means of code
        </div>
      </div>
      <div></div>
      <div></div>
    </div>
    <div className="w-full font-body text-3xl my-10 flex justify-evenly px-2 flex-wrap">
      <div className="w-full text-xl max-w-sm">
        {`Hey there, I'm Kevin! I like rust, kubernetes, virtual reality, bevy,
      audio synthesis, and many other things. I have built lots of infrastructure, from big data
      analytics to realtime video processing. In my spare time I am branching out into
      3D working with the bevy game engine.`}
      </div>
      <div></div>
      <div></div>
    </div>
  </>
);

export default Home;
