[How to export your Bevy project to the web with WASM](https://www.youtube.com/watch?v=VjXiREbPtJs)

rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/game.wasm

mkdir webbuild
cp -r out webbuild
cp -r asset webbuild
cd webbuild
touch index.html
```
<html>
  <head>
    <meta charset="UTF-8" />
    <style>
      body {
        margin: 0;
        background: darkgrey;
        background-size: 400% 400%;
        animation: gradient 15s ease infinite;
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;    
      }
      .loader {
        border: 16px solid #f3f3f3;
        border-radius: 50%;
        border-top: 16px solid #3498db;
        width: 120px;
        height: 120px;
        position: absolute;
        z-index : -999;
        -webkit-animation: spin 2s linear infinite;
        animation: spin 2s linear infinite;
      }
      
      @-webkit-keyframes spin {
        0% { -webkit-transform: rotate(0deg); }
        100% { -webkit-transform: rotate(360deg); }
      }
      
      @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
      }
      </style>
  </head>
  <body>
    <div class="loader"></div>
    <script>
        // the following function keeps track of all AudioContexts and resumes them on the first user
        // interaction with the page. If the function is called and all contexts are already running,
        // it will remove itself from all event listeners.
        (function () {
            // An array of all contexts to resume on the page
            const audioContextList = [];

            // An array of various user interaction events we should listen for
            const userInputEventNames = [
                "click",
                "contextmenu",
                "auxclick",
                "dblclick",
                "mousedown",
                "mouseup",
                "pointerup",
                "touchend",
                "keydown",
                "keyup",
            ];

            // A proxy object to intercept AudioContexts and
            // add them to the array for tracking and resuming later
            self.AudioContext = new Proxy(self.AudioContext, {
                construct(target, args) {
                    const result = new target(...args);
                    audioContextList.push(result);
                    return result;
                },
            });

            // To resume all AudioContexts being tracked
            function resumeAllContexts(_event) {
                let count = 0;

                audioContextList.forEach((context) => {
                    if (context.state !== "running") {
                        context.resume();
                    } else {
                        count++;
                    }
                });

                // If all the AudioContexts have now resumed then we unbind all
                // the event listeners from the page to prevent unnecessary resume attempts
                // Checking count > 0 ensures that the user interaction happens AFTER the game started up
                if (count > 0 && count === audioContextList.length) {
                    userInputEventNames.forEach((eventName) => {
                        document.removeEventListener(eventName, resumeAllContexts);
                    });
                }
            }

            // We bind the resume function for each user interaction
            // event on the page
            userInputEventNames.forEach((eventName) => {
                document.addEventListener(eventName, resumeAllContexts);
            });
        })();
    </script>
    <script type="module">
        import init from '/out/project_name_here.js'
        init();
    </script>
</body>
</html>
```

npx serve .
