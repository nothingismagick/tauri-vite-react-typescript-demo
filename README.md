# tauri-vite-react-typescript-demo

```
npx create-tauri-app
```

was used to scaffold this repo, choosing:

- vite
- react
- typescript

## Presenter Notes

- [main.rs](src-tauri/src/main.rs) shows a Rust command that creates a new window using the `WindowBuilder`
- [App.tsx](src/App.tsx) uses `invoke` to call that command in response to a button click
- The fancy overlay window opens showing that Tauri windows are very versatile
- [Overlay.tsx](src/Overlay.tsx) makes use of the `@tauri-apps/api/window` module to close the overlay window again (using `getCurrent` and `WebviewWindow.close()`)
- [vite.config.ts](vite.config.ts) has some special configuration to make the Vite + Tauri DX nicer