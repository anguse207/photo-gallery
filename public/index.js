const uploadUrl = "/api/upload";

const form = document.forms.namedItem("fileinfo");
form.addEventListener(
  "submit",
  (event) => {
    console.log("Trying to upload file...");
    const output = document.querySelector("#output");
    const formData = new FormData(form);

    const request = new XMLHttpRequest();
    request.open("POST", uploadUrl, true);
    request.onload = (progress) => {
      output.innerHTML =
        request.status === 200
          ? "200 OK"
          : `${request.status}`;
    }
    request.send(formData);
    event.preventDefault();
  },
  false,
);