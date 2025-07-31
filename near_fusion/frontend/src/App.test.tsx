import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0);

  return (
    <div className="min-h-screen bg-gray-100 p-8">
      <h1 className="text-3xl font-bold text-center mb-8">NEAR Fusion+ Test</h1>
      <div className="max-w-md mx-auto bg-white rounded-lg shadow p-6">
        <p className="text-lg mb-4">Basic React app is working!</p>
        <button 
          onClick={() => setCount(count + 1)}
          className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
        >
          Count: {count}
        </button>
      </div>
    </div>
  )
}

export default App