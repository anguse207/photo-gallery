import { useEffect, useState } from "react";

import { url_ws } from "./consts";

const ConnectWs = (url: string) => {
  const [images, setImages] = useState<string[]>([]);

  useEffect(() => {
    const ws = new WebSocket(url);

    ws.onmessage = (event) => {
      setImages((prev) => [...prev, event.data]); // Add new image to collection
    };

    return () => ws.close();
  }, [url]);

  return images;
};

const Gallery = () => {
  const images = ConnectWs(url_ws);
  const [currentIndex, setCurrentIndex] = useState(0);

  useEffect(() => {
    if (images.length > 0) {
      const interval = setInterval(() => {
        setCurrentIndex((prev) => (prev + 1) % images.length);
      }, 30000); // Change image every 30s

      return () => clearInterval(interval);
    }
  }, [images]);

  return (
    <div className="flex justify-center items-center h-screen bg-gray-900">
      {images.length > 0 ? (
        <img src={images[currentIndex]} alt="Gallery" className="max-w-full max-h-full rounded-lg shadow-lg" />
      ) : (
        <p className="text-white">Waiting for images...</p>
      )}
    </div>
  );
};

export default Gallery;
