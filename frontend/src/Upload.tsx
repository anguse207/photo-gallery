import React, { useRef, useState } from 'react';

import Button from '@mui/material/Button';

const upload_url = "http://127.0.0.1:3000/upload";

const Upload: React.FC = () => {
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const handleButtonClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    console.log(upload_url);

    if (!event.target.files || event.target.files.length === 0) return;

    const files = Array.from(event.target.files);

    for (const file of files) {
      const formData = new FormData();
      formData.append("files", file); // Adjust field name as per your API

      try {
        const response = await fetch(upload_url, {
          method: "POST",
          body: formData,
        });

        // console.log(response);
        if (!response.ok) throw new Error("Upload failed");
      } catch (error) {
        console.error(`Error uploading file "${file.name}":`, error);
      }
    }

    event.target.value = ""; // Reset input to allow re-selection
  };

  return (
    <div>
      <Button variant='contained' onClick={handleButtonClick}>Upload File</Button>
      <input
        type="file"
        ref={fileInputRef}
        onChange={handleFileChange}
        multiple
        style={{ display: "none" }}
      />
    </div>
  );
};

export default Upload;

