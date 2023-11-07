import './app.module.scss'
import { Format, scan } from "@tauri-apps/plugin-barcode-scanner";

export function App() {
  return (
    <>
      <button onClick={(ev) => {
        try {
          scan({
            windowed: true,
            formats: [Format.QRCode],
            cameraDirection: "back",
          })
        } catch (e) {
          console.error(e)
        }
      }}>
        Scan
      </button>
    </>
  )
}