// import { useState } from "react";
import "./App.css";
import { Link, Route, Routes } from "react-router-dom";
import Home from "./pages/Home";
import Map from "./pages/Map";

function App() {

  return (
    <div>
      <nav>
        <Link to="/">Home</Link> | <Link to="/map">Map</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/map" element={<Map />} />
      </Routes>
    </div>
  );
}

export default App;
