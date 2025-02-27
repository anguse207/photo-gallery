import { useEffect } from "react";
import QRCode from "qrcode";

import "./QR.css";

function generateQRCode() {
  const hostUrl = window.location.origin; // Gets the current host

  QRCode.toCanvas(hostUrl, { width: 200 }, (err, canvas) => {
    if (err) {
      console.error(err);
      return;
    }
    const qrContainer = document.getElementById("qrcode");
    if (qrContainer) {
      qrContainer.innerHTML = "";
      qrContainer.appendChild(canvas);
    }
  });
}

function QR() {
  useEffect(() => {
    generateQRCode();
  }, []);

  return <div id="qrcode"></div>;
}

export default QR;
