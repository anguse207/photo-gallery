import { useEffect } from "react";

import { url_ws } from "./consts";

import './Gallery.css'
 

const ConnectWs = () => {
  useEffect(() => {
    const ws = new WebSocket("ws://127.0.0.1:3000/api/ws");

    ws.onmessage = (event) => {
      // set the image source to the received data
      const img = document.getElementById("gallery") as HTMLImageElement;

      const blob = new Blob([event.data], { type: "image/" });
      const url = URL.createObjectURL(blob);
      img.src = url;
    };
  }, []);
};

const Gallery = () => {
  ConnectWs();

  return (
    <>
      <h1>Gallery</h1>
      <img id="gallery" />
    </>
  );
};

export default Gallery;
