import { create } from "zustand";
import { devtools } from "zustand/middleware";
import {nth_prime} from "../../pkg";


export const useAppStore = create(
    devtools(
        (set) => ({
                    greetFn: (str) => "Hello from WebAssembly"+str,
                    nthPrime:(n) => "0",
                    setStoreGreetFn: (fn) => set({ greetFn: fn }),
                    setStoreNthPrimeFn: (fn) => set({ nthPrime: fn })
        })
    )
);
