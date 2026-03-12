import { Bench } from "tinybench";

import { getSoundDevices, getMute, setMute, getSystemVolume, setSystemVolume } from "../index.js";

const bench = new Bench();

bench.add("getSoundDevices", () => {
    getSoundDevices();
});

bench.add("getMute", () => {
    getMute();
});

bench.add("setMute", () => {
    setMute(false);
});

bench.add("getSystemVolume", () => {
    getSystemVolume();
});

bench.add("setSystemVolume", () => {
    setSystemVolume(50);
});

await bench.run();

console.table(bench.table());
