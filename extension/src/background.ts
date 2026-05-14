chrome.runtime.onMessage.addListener((message) => {
  if (message.type === "TABS_CAPTURED") {
    console.log("Received tabs:", message.tabs.length);
  }
});

export {};
