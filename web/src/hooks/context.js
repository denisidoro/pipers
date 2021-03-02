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
                const convert = (x) => {
                    try {
                        return w.convert(x)
                    }
                    catch (err) {
                        console.log(err)
                        return err.toString()
                    }
                }
                setData(data => ({ ...data, convert }))
            } catch (err) {
                console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
            }
        }
        fetch()
    }, []);

    return data
}

export const useMainContext = () => useContext(MainContext)