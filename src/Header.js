import React, { useEffect, useState } from "react";
import { useAppStore } from "./store/appStore";
import { parallel_wasm,fetch_parallel } from "../pkg";
import { parallelJs,fetchParallel } from "./utils";

export const Header = () => {
    const wasmFetch = () => {
         parallel_wasm(1000000000);
        //parallelJs(1000000000);
        // fetch_parallel(100).then(r => {
        //     console.log("Wasm Done");
        // });
        
        // fetchParallel(1000).then(r => {
        //     console.log("JS Done");
        // });
    }
    const jsFetch = () => {
        // parallel_wasm(1000000000);
        //parallelJs(1000000000);
      
        fetchParallel(100).then(r => {
            console.log("JS Done");
        });
    }

    return <div style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-around",
        alignItems: "center",
        gap: "10px",
        height: "50%",
        width: "50%"
    }}><button onClick={() => { wasmFetch() }}>Fetch WASM</button>
        <button onClick={() => { jsFetch() }}>Fetch JS</button></div>;

}