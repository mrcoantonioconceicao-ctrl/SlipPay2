#!/data/data/com.termux/files/usr/bin/bash

cd ~/slippay-rust

git add .

DATE=$(date "+%Y-%m-%d %H:%M:%S")

git commit -m "auto update $DATE"

git push origin main

echo "🚀 SlipPay enviado para GitHub!"
