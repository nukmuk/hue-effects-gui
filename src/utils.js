import { useState } from "react";

function getFromStorage(key) {
    const value = window.localStorage.getItem(key);
    console.log("get from storage", key, value);
    try {
        return JSON.parse(value);
    } catch (error) {
        return value;
    }
}

function useLocalStorage(key, defaultValue) {
    return useState(getFromStorage(key) || defaultValue);
}

export { getFromStorage, useLocalStorage };