import { ColorPicker, Input } from '@mantine/core'
import { invoke } from '@tauri-apps/api';
import React, { useState } from 'react'
import { useEffect } from 'react';
import { useLocalStorage } from '../../utils';

export default function SolidControls() {

    const [color, setColor] = useLocalStorage("color", '#ffffff');

    useEffect(() => {
        localStorage.setItem("color", color);
        invoke("set_solid_color", { color });
    }, [color])


    return (
        <ColorPicker value={color} onChange={setColor} size='lg' />
    )
}
