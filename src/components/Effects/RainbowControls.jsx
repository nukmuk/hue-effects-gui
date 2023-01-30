import React, { useEffect, useState } from 'react'
import { Slider } from '@mantine/core'
import { invoke } from '@tauri-apps/api';

export default function RainbowControls() {

    const [scale, setScale] = useState(0.5);
    const [speed, setSpeed] = useState(5);
    const [angle, setAngle] = useState(0);

    useSetRainbowProperty(scale, "scale");
    useSetRainbowProperty(speed, "speed");
    useSetRainbowProperty(angle, "angle");

    function useSetRainbowProperty(property, type) {
        useEffect(() => {
            invoke("edit_rainbow", { [type]: property });
        }, [property]);  // todo add localStorage to scale, speed etc
    }

    return (
        <>
            Scale
            < Slider size={"lg"} color={"yellow"} defaultValue={50} max={150} onChange={e => setScale(e / 100)} />
            Speed
            < Slider size={"lg"} color={"indigo"} defaultValue={5} max={150} onChange={setSpeed} />
            Angle
            < Slider size={"lg"} color={"red"} defaultValue={25} max={360} onChange={setAngle} disabled="true" />
        </>
    )
}
