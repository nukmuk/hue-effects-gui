import { Canvas, useFrame, useThree, extend } from '@react-three/fiber'
import { useHelper, OrthographicCamera } from '@react-three/drei'
import React, { useRef, useState } from 'react'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls'
import { BackSide, Color, FogExp2, PointLightHelper } from 'three'
import { appWindow } from '@tauri-apps/api/window'
import { Bloom, DepthOfField, EffectComposer, Noise, Vignette } from '@react-three/postprocessing'

extend({ OrbitControls })

let lightRefs = [];
let lightHelperRefs = [];

export default function RoomPreview(props) {

    function Box(props) {
        // This reference gives us direct access to the THREE.Mesh object
        const ref = useRef()
        // Hold state for hovered and clicked events
        const [hovered, hover] = useState(false)
        const [clicked, click] = useState(false)
        // Subscribe this component to the render-loop, rotate the mesh every frame
        // useFrame((state, delta) => (ref.current.rotation.y += delta))
        // Return the view, these are regular Threejs elements expressed in JSX
        return (
            <mesh
                {...props}
                ref={ref}>
                <boxGeometry args={[4.1, 2.1, 4.1]} />
                <meshLambertMaterial color={"white"} side={BackSide} />
            </mesh>
        )
    }

    function Controls() {
        const controls = useRef()
        const { camera, gl } = useThree()
        useFrame(() => controls.current.update())
        return <orbitControls ref={controls} args={[camera, gl.domElement]} enableDamping dampingFactor={0.05} rotateSpeed={0.1}
            enablePan={false} enableZoom={false} minAzimuthAngle={0} maxAzimuthAngle={Math.PI / 2} maxPolarAngle={Math.PI / 2} />
    }

    function Env() {
        useFrame((state, delta) => {
            // state.camera.position.x = state.mouse.x;
            // state.camera.position.y = state.mouse.y;
        });


    }

    function Lights() { // todo add ent area selector : D
        try {
            const channels = props.areas[1].channels;
            console.log("propsit: ", channels);
            let lights = [];
            lightRefs = [];
            channels.forEach(channel => {
                const light = useRef();
                const helper = useHelper(light, PointLightHelper, 0.1);
                lights.push(<pointLight ref={light} position={[channel.position.x * 2, channel.position.z, -channel.position.y * 2]} color={"white"} distance={4} intensity={.1} />)
                lightRefs.push(light);
                lightHelperRefs.push(helper);
            });

            return lights;
        } catch (error) {
            console.log("error: ", error);
        }
    }

    function rgbToHexTriplet(r, g, b) {
        r = r & 255;
        g = g & 255;
        b = b & 255;
        console.log("rgb: ", r, g, b)
        return parseInt([r, g, b].map(x => x.toString(16).padStart(2, '0')).join(''), 16);
    }

    appWindow.listen("lightUpdate", event => {
        const payload = event.payload;
        console.log("light update", payload);
        payload.forEach(light => {
            const lightRef = lightRefs[light.id];
            const hexTriplet = rgbToHexTriplet(...light.color);
            console.log("hexTriplet: ", hexTriplet);
            lightRef.current.color.set(hexTriplet);
        });
    });


    return (
        <Canvas style={{ height: "400px", width: "500px", marginBottom: "4px" }} // todo: doesn't shrink when shrinking window ?
        // camera={{ position: [40, 20, 40], fov: 5, }}
        >
            <OrthographicCamera makeDefault position={[20, 10, 20]} zoom={80} />
            <ambientLight intensity={0.015} />
            {/* <fogExp2 color={"#141517"} density={0.02} far={190} near={1} /> */}
            {/* <fog args={["#141517", 60, 70]} attach={"fog"} /> */}
            <fog args={["#141517", 35, 40]} attach={"fog"} />
            <Lights />
            <Box position={[0, 0, 0]} />
            <Env />
            <gridHelper position={[0, -1.2, 0]} args={[17, 16]} />
            <Controls />
            {/* <OrbitControls /> */}
            <EffectComposer>
                {/* <DepthOfField target={[0, 0, 0]} focalLength={0.02} bokehScale={2} height={480} /> */}
                <Bloom mipmapBlur intensity={1} />
                {/* <Noise opacity={0.02} /> */}
            </EffectComposer>
        </Canvas >
    );
}
