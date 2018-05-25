#!/usr/bin/env bash
ffmpeg -i out/%d.png -vcodec libx264 -pix_fmt yuv420p -framerate 24 -s 1024x1024 -sws_flags neighbor output.mp4