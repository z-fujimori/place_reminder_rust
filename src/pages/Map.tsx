// import React from 'react'

import { MapContainer, Marker, Popup, TileLayer } from "react-leaflet"

const Map = () => {
  return (
    <div >
      <div>Mp</div>
      <div>
        <MapContainer center={[35.6895, 139.6917]} zoom={10} style={{ height: "700px", width: "100%" }}>
          <TileLayer
            attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png"
            // url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png" // ダークモード
          />
          <Marker position={[35.6895, 139.6917]}>
            <Popup>
              東京
            </Popup>
          </Marker>
        </MapContainer>
      </div>
    </div>
  )
}

export default Map