cross build --target=arm-unknown-linux-musleabihf && \
scp target/arm-unknown-linux-musleabihf/debug/pi@raspberrypi.local:~/build 
# && ssh pi@raspberrypi.local ./oled