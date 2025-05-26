import { useAppStore } from "../store/appStore";
import { greet, nth_prime } from "../../pkg";
import { useEffect } from "react";

export const useStoreWasm = () => {
    // const setGreetFn = useAppStore((state) => state.setStoreGreetFn);
    // const setNthPrimeFn = useAppStore((state) => state.setStoreNthPrimeFn);

    // useEffect(() => {
    //     setGreetFn(greet);
    //     setNthPrimeFn(nth_prime);
    // }, []);
}