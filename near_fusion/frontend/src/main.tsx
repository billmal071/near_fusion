import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Buffer } from 'buffer'
import './index.css'
// Comment out to switch between single-chain and cross-chain versions
// import App from './App.tsx'
import CrossChainApp from './CrossChainApp.tsx'
import { Web3Provider } from './providers/Web3Provider.tsx'

// Polyfill for NEAR API JS
window.Buffer = Buffer

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Web3Provider>
      <CrossChainApp />
    </Web3Provider>
  </StrictMode>,
)