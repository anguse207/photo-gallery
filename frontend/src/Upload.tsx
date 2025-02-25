import React, { useRef } from 'react';

import Button from '@mui/material/Button';

import { url_upload } from './consts';


const Upload: React.FC = () => {
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const handleButtonClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.files || event.target.files.length === 0) return;

    const files = Array.from(event.target.files);

    for (const file of files) {
      const formData = new FormData();
      formData.append("user-image_upload", file);

      try {
        const response = await fetch(
          url_upload, 
          {
            method: "POST",
            body: formData,
          }
        );

        // console.log(response);
        if (!response.ok) throw new Error("Upload failed");
      } catch (error) {
        console.error(`Error uploading file "${file.name}":`, error);
      }
    }

    event.target.value = "";
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

