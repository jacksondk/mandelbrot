console.log("test");
import("./index.js")
    .catch(e => console.error("Error importing `index.js`:", e));