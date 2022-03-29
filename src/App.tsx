import logos from './logos.svg'
import { invoke } from '@tauri-apps/api'

export default function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logos} className="logos" alt="logos" />
        <p>Hello Tauri + Vite + React!</p>
        <p>
          <button type="button" onClick={() => invoke('open_overlay_window')}>
            Click Me!
          </button>
        </p>
      </header>
    </div>
  )
}