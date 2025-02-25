import './App.css'

import { useState } from 'react'
import Button from '@mui/material/Button'

import QR from './QR'
import Upload from './Upload'
import Gallery from './Gallery'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <Button variant="contained" onClick={() => setCount((count) => count + 1)}>
        count is {count}
      </Button>
      <Upload />
      
      <Gallery />

      <div id='qrcode'>
        <QR />
      </div>
    </>
  )
}

export default App
