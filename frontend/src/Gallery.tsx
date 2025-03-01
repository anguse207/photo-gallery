import { useEffect, useState, useRef } from "react";

import "./Gallery.css";

import { url_ws } from "./consts";

const useWebSocket = (url: string) => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const reconnectRef = useRef(0); // Track reconnection attempts

  useEffect(() => {
    const connect = () => {
      const socket = new WebSocket(url);

      socket.onopen = () => {
        console.log("Connected to WebSocket");
        reconnectRef.current = 0; // Reset retries on successful connection
      };

      socket.onmessage = async (event) => {
        const gallery = document.getElementById("gallery") as HTMLDivElement;
        const blob = new Blob([event.data], { type: "image/png" });
        const url = URL.createObjectURL(blob);

        gallery.classList.add("transition");
        await new Promise((res) => setTimeout(res, 1000));
        gallery.classList.remove("transition");

        gallery.style.backgroundImage = `url(${url})`;

        gallery.classList.add("hide-above");
        await new Promise((res) => setTimeout(res, 10));
        gallery.classList.remove("hide-above");
      };

      socket.onclose = () => {
        console.log("WebSocket closed. Reconnecting...");
        reconnectRef.current += 1;
        setTimeout(connect, Math.min(1000 * 2 ** reconnectRef.current, 30000)); // Exponential backoff
      };

      socket.onerror = (err) => {
        console.error("WebSocket error:", err);
        socket.close();
      };

      setWs(socket);
    };

    connect();
    return () => ws?.close();
  }, [url]);

  return ws;
};

const Gallery = () => {
  useWebSocket(url_ws);

  return <div id="gallery" />;
};

export default Gallery;
