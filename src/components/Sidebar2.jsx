import { Button, Flex } from '@mantine/core';
import { useEffect } from 'react';
import { Link, NavLink, useLocation } from 'react-router-dom';
// import "./sidebar.css";

export default function Sidebar2() {

    const currentPath = useLocation().pathname;

    return <>
        <Flex
            direction="column"
            gap="md"
        >
            <Button component={NavLink} to="/" variant={currentPath == "/" ? "gradient" : "default"} gradient={{ from: 'red', to: 'orange' }}>Effects</Button>
            <Button component={NavLink} to="/lights" variant={currentPath === "/lights" ? "gradient" : "default"} gradient={{ from: 'indigo', to: 'cyan' }}>Lights</Button>
            <Button component={NavLink} to="/bridges" variant={currentPath === "/bridges" ? "gradient" : "default"} gradient={{ from: 'red', to: 'pink' }} >Bridges</Button>
        </Flex >
    </>;
}
