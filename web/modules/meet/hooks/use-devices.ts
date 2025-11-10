import { useState, useEffect } from "react";

export const useDevices = () => {
  const [audioDevices, setAudioDevices] = useState<MediaDeviceInfo[]>([]);
  const [videoDevices, setVideoDevices] = useState<MediaDeviceInfo[]>([]);

  useEffect(() => {
    const handleDevices = async () => {
      let devices = await navigator.mediaDevices.enumerateDevices();

      let audioD: MediaDeviceInfo[] = [];
      let videoD: MediaDeviceInfo[] = [];

      devices.map((device) => {
        if (device.kind === "videoinput") {
          videoD.push(device);
        }

        if (device.kind === "audioinput") {
          audioD.push(device);
        }
      });

      if (audioD.length) setAudioDevices(audioD);
      if (videoD.length) setVideoDevices(videoD);
    };

    navigator.mediaDevices.addEventListener("devicechange", handleDevices);

    return () => {
      navigator.mediaDevices.removeEventListener("devicechange", handleDevices);
    };
  }, []);

  return { audioDevices, videoDevices };
};
