killall _15 actix_demo_tokio axum_demo_tokio demo_salvo_tokio actix_demo_actix  spring_rs_demo_tokio axum_demo_actix demo_salvo_actix

TestFile=spring_rs_demo_tokio

nohup  ./$TestFile >/dev/null 2>&1 &
until [ ! -z "$(pidof $TestFile)" ]; do sleep 0.1s; done
clear
echo "当前是: $TestFile"
/data/data/com.termux/files/usr/bin/ab -n 50000 -c 9999 http://127.0.0.1:5800/
