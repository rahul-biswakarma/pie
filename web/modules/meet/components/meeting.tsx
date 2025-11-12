"use client";

import { useRef } from "react";

import { useMediaStream } from "../hooks/use-media-stream";

export const MeetingPage = () => {
  const videoContainerRef = useRef<HTMLVideoElement>(null);

  const {
  videoDevices,
    audioDevices,
    selectedVideoDevice,
    selectedAudioDevice,
  } = useMediaStream(videoContainerRef);

  return (
    <div>
      Meeting
      <video
        className="transform -scale-x-100"
        width={500}
        height={300}
        playsInline
        autoPlay
        ref={videoContainerRef}
      />
    </div>
  );
};
