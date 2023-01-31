import React, { useEffect, useState } from 'react'
import { Slider } from '@mantine/core'
import { invoke } from '@tauri-apps/api';
import { getFromStorage } from '../../utils.js';

export default function RainbowControls(props) {

    const [scale, setScale] = useState(getFromStorage("scale", 1.2));
    const [speed, setSpeed] = useState(getFromStorage("speed", 5));
    const [angle, setAngle] = useState(getFromStorage("angle", 25));

    useSetRainbowProperty(scale, "scale");
    useSetRainbowProperty(speed, "speed");
    useSetRainbowProperty(angle, "angle");

    function useSetRainbowProperty(value, type) {
        useEffect(() => {
            console.log("setting rainbow property", type, value)
            window.localStorage.setItem(type, value);
            invoke("edit_rainbow", { [type]: value });
        }, [value]);
    }

    return (
        <>
            Scale
            < Slider value={scale} size={"lg"} color={"yellow"} defaultValue={50} max={150} onChange={e => setScale(e)} /*marks={[{ value: 50, label: "Default" }]}*/ />
            Speed
            < Slider value={speed} size={"lg"} color={"indigo"} defaultValue={5} max={150} onChange={setSpeed} />
            Angle
            < Slider value={angle} size={"lg"} color={"red"} defaultValue={25} max={360} onChange={setAngle} disabled="true" />
        </>
    )
}
