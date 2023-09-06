# Rust/Axum/Htmx/AlpineJs/Tera/Tailwind Web Dev Yeehaw

## To Build Javascript Dependencies (Htmx and Alpine Js)
```
npm install
npm run build
```
esbuild is used to bundle all dependencies. Dependencies can be added to app.js and they will be bundle automatically.


## For Tailwind CSS Purging/Watching: 
```
npx tailwindcss -i ./ui/assets/tailwind.css -o ./dist/output.css --watch
```

## For Hot Reloading
1. Install cargo watch [watchtower](https://github.com/watchexec/cargo-watch)
2. Run
```
cargo watch -x run
```
