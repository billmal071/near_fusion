import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Buffer } from 'buffer'
import './index.css'
// Comment out to switch between single-chain and cross-chain versions
// import App from './App.tsx'
import CrossChainApp from './CrossChainApp.tsx'
import { Web3Provider } from './providers/Web3Provider.tsx'

// Polyfills for NEAR API JS
window.Buffer = Buffer
// @ts-ignore
window.process = window.process || { env: {} }

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Web3Provider>
      <CrossChainApp />
    </Web3Provider>
  </StrictMode>,
)