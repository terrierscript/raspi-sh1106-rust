cross build --target=arm-unknown-linux-musleabihf && \
scp target/arm-unknown-linux-musleabihf/debug/oled pi@raspberrypi.local:~/ && \
ssh pi@raspberrypi.local ./oled