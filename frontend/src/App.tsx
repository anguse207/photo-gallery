import { useState } from 'react'
import './App.css'

import QR from './QR'
import Upload from './Upload'
import Button from '@mui/material/Button'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <h1>Vite + React</h1>
      <div className="card">
        <Button variant="contained" onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </Button>
      <Upload />
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>

      <div id='qrcode'>
        <QR />
      </div>
    </>
  )
}

export default App
