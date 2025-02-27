import { useEffect } from "react";

import { url_ws } from "./consts";

import './Gallery.css'

function timeout(delay: number) {
  return new Promise( res => setTimeout(res, delay) );
}

const ConnectWs = () => {
  useEffect(() => {
    const ws = new WebSocket(url_ws);

    ws.onmessage = async (event) => {
      const gallery = document.getElementById("gallery") as HTMLImageElement;
      const blob = new Blob([event.data], { type: "image/" });
      const url = URL.createObjectURL(blob);

      // Hide current image
      gallery.classList.add("transition");
      await timeout(999);
      gallery.classList.remove("transition");

      // Set new image
      gallery.style.backgroundImage = `url(${url})`;

      // Add transition effect
      gallery.classList.add("hide-above");
      await timeout(1);
      gallery.classList.remove("hide-above");
    };
  }, []);
};

const Gallery = () => {
  ConnectWs();

  return (
    <>
      <div id="gallery" />
    </>
  );
};

export default Gallery;
