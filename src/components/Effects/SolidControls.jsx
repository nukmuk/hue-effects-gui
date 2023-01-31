import { ColorPicker, Input } from '@mantine/core'
import { invoke } from '@tauri-apps/api';
import React, { useState } from 'react'
import { useEffect } from 'react';

export default function SolidControls() {

    const [color, setColor] = useState('#000000'); // todo: color isnt' persistent yet

    useEffect(() => {
        invoke("set_solid_color", { color });
    }, [color])


    return (
        <ColorPicker onChange={setColor} size='lg' />
    )
}
