async function loadUrl(url) {
  await fetch("wry://load_url", {
    method: "POST",
    body: JSON.stringify({ url }),
    headers: { "Content-Type": "application/json" }
  });
}


addEventListener("load", () => {
  setupUrlBox();
});


function setupUrlBox() {
  let urlInput = document.getElementById("url-input");
  let navigateButton = document.getElementById("navigate-button");

  const validateUrl = (text) => text.startsWith("http://") || text.startsWith("https://");

  navigateButton.addEventListener("click", () => {
    if (validateUrl(urlInput.value)) {
      loadUrl(urlInput.value);
    } else {
      alert("そのURLは無効です。");
    }
  });
}
