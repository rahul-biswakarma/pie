import { useEffect, useState } from "react";
import { useDevices } from "./use-devices";

export const useDeviceHandler = () => {
  const [selectedVideoDevice, setSelectedVideoDevice] =
    useState<MediaDeviceInfo | null>(null);
  const [selectedAudioDevice, setSelectedAudioDevice] =
    useState<MediaDeviceInfo | null>(null);

  const { videoDevices, audioDevices } = useDevices();

  useEffect(() => {
    if (
      !selectedVideoDevice ||
      (selectedVideoDevice &&
        !videoDevices.find(
          (device) => device.deviceId === selectedVideoDevice.deviceId,
        ))
    ) {
      setSelectedVideoDevice(videoDevices[0]);
    }
    if (
      !selectedAudioDevice ||
      (selectedAudioDevice &&
        !audioDevices.find(
          (device) => device.deviceId === selectedAudioDevice.deviceId,
        ))
    ) {
      setSelectedAudioDevice(audioDevices[0]);
    }
  }, [videoDevices, audioDevices]);

  return {
    selectedAudioDevice,
    selectedVideoDevice,
    videoDevices,
    audioDevices,
  };
};
