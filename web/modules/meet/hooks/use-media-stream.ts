import { RefObject, useEffect, useRef } from "react";
import { useDeviceHandler } from "./use-device-handler";

export const useMediaStream = (
  videoContaienrRef: RefObject<HTMLVideoElement | null>,
) => {
  const {
    videoDevices,
    audioDevices,
    selectedVideoDevice,
    selectedAudioDevice,
  } = useDeviceHandler();

  const streamRef = useRef<MediaStream | null>(null);

  useEffect(() => {
    const start = async () => {
      let stream = await navigator.mediaDevices.getUserMedia({
        video: {
          deviceId: selectedVideoDevice?.deviceId,
        },
        audio: {
          deviceId: selectedAudioDevice?.deviceId,
        },
      });
      streamRef.current = stream;
      if (videoContaienrRef.current) {
        videoContaienrRef.current.srcObject = stream;
      }
    };

    const stop = async () => {
      let stream = streamRef.current;

      if (!stream) return;
      stream?.getTracks().forEach((t) => t.stop());
      streamRef.current = null;
    };

    start();

    return () => {
      stop();
    };
  }, []);

  return {
    streamRef,
    videoDevices,
    audioDevices,
    selectedVideoDevice,
    selectedAudioDevice,
  };
};
