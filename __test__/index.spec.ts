import test from "ava";

import { getSoundDevices, getMute, setMute, getSystemVolume, setSystemVolume } from "../index.js";

test.serial("#getSoundDevices()", (t) => {
    const devices = getSoundDevices();
    t.true(Array.isArray(devices), "devices is an array");
    t.true(
        devices.every((d) => typeof d === "string"),
        "devices is an array of strings",
    );
});

test.serial("#getMute()", (t) => {
    const mute = getMute();
    t.is(typeof mute, "boolean");
});

test.serial("#setMute(bool)", (t) => {
    const mute = getMute();
    setMute(!mute);

    const new_mute = getMute();
    t.not(mute, new_mute, "mute state shoud change");
});

test.serial("#getSystemVolume()", (t) => {
    const volume = getSystemVolume();
    t.is(typeof volume, "number");
    t.true(volume >= 0 && volume <= 100, "volume should be a percentage");
});

test.serial("#setSystemVolume(number)", (t) => {
    const volume = getSystemVolume();
    const expected_volume = volume + 10 > 100 ? volume - 10 : volume + 10;
    setSystemVolume(expected_volume);

    const new_volume = getSystemVolume();
    t.is(new_volume, expected_volume, "volume should match new volume");
});

test.serial("#setSystemVolume() throws if percent is incorrect", (t) => {
    t.throws(() => {
        setSystemVolume(120);
    });

    t.throws(() => {
        setSystemVolume(-1);
    });
});
