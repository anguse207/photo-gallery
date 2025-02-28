export const url_upload = window.location.origin + "/api/upload";
export const url_ws = "ws://" + window.location.hostname + "/api/ws";

console.log("url_upload: ", url_upload);
console.log("url_ws: ", url_ws);

/* Examples...
window.location.origin
"https://mail.google.com"

window.location.host
"mail.google.com"

window.location.hostname
"mail.google.com"

window.location.protocol
"https:" 
*/