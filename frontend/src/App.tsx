import './App.css'

import QR from './QR'
import Upload from './Upload'
import Gallery from './Gallery'

function App() {
  return (
    <>
      <Upload />
      <Gallery />
      <div id='qrcode'>
        <QR />
      </div>
    </>
  )
}

export default App
