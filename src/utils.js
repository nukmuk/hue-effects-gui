function getFromStorage(key) {
    const value = window.localStorage.getItem(key);
    console.log("get from storage", key, value);
    try {
        return JSON.parse(value);
    } catch (error) {
        return value;
    }
}

export { getFromStorage };