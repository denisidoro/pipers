import React, { createContext, useContext, useEffect, useState, useCallback } from 'react'

const initialState = {
    convert: (x) => "",
}

export const MainContext = createContext(initialState)

export function useValue() {
    const [data, setData] = useState(initialState);

    useEffect(() => {
        async function fetch() {
            try {
                const w = await import("pipers");
                setData(data => ({ ...data, convert: w.convert }))
            } catch (err) {
                console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
            }
        }
        fetch()
    }, []);

    return data
}

export const useMainContext = () => useContext(MainContext)