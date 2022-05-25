import * as THREE from "three";
import * as React from "react";
import { useRef, useState } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls, Stars } from "@react-three/drei";

const Line = () => (
  <group position={[0, 0, 0]}>
    <mesh>
      <boxBufferGeometry attach="geometry" args={[1, 1, 1]} />
      <meshStandardMaterial attach="material" color={0xf95b3c} />
    </mesh>
  </group>
);

function App() {
  return (
    <Canvas style={{ width: "100vw", height: "100vh", background: "#222" }}>
      <Stars />
      <OrbitControls></OrbitControls>
      <ambientLight intensity={0.5} />
      <spotLight position={[10, 15, 10]} angle={0.3} />
      <Line />
    </Canvas>
  );
}

export default App;
