import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './App'
import Overlay from './Overlay'

ReactDOM.render(
  <React.StrictMode>
    {location.pathname === '/overlay' && <Overlay />}
    {location.pathname === '/' && <App />}
  </React.StrictMode>,
  document.getElementById('root')
)
