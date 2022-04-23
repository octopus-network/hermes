#!/usr/bin/env bash

#createclient
echo "########################################################"
echo "hermes -c config.toml tx raw create-client ibc-0 ibc-1"
echo "########################################################"
hermes -c config.toml tx raw create-client ibc-0 ibc-1            
# ./target/debug/hermes -c config.toml query client state ibc-0 10-grandpa-0       
echo "########################################################"
echo "hermes -c config.toml tx raw create-client ibc-1 ibc-0"
echo "########################################################"
hermes -c config.toml tx raw create-client ibc-1 ibc-0            
# ./target/debug/hermes -c config.toml query client state ibc-1  10-grandpa-0   
sleep 5
#updateclient
echo "########################################################"
echo "hermes -c config.toml tx raw update-client ibc-0 10-grandpa-0"
echo "########################################################"
hermes -c config.toml tx raw update-client ibc-0 10-grandpa-0   

echo "########################################################"
echo "hermes -c config.toml query client state ibc-0 10-grandpa-0"
echo "########################################################"
hermes -c config.toml query client state ibc-0 10-grandpa-0      

echo "########################################################"
echo "hermes -c config.toml tx raw update-client ibc-1  10-grandpa-0"
echo "########################################################" 
hermes -c config.toml tx raw update-client ibc-1  10-grandpa-0 

echo "########################################################"
echo "hermes -c config.toml query client state ibc-1  10-grandpa-0"
echo "########################################################"
hermes -c config.toml query client state ibc-1  10-grandpa-0   


#connection
echo "########################################################"
echo "hermes -c config.toml tx raw conn-init ibc-0 ibc-1 10-grandpa-0 10-grandpa-0"
echo "########################################################"
hermes -c config.toml tx raw conn-init ibc-0 ibc-1 10-grandpa-0 10-grandpa-0
sleep 5
echo "########################################################"
echo "hermes -c config.toml tx raw conn-try ibc-1 ibc-0 10-grandpa-0 10-grandpa-0 -s connection-0"
echo "########################################################"
hermes -c config.toml tx raw conn-try ibc-1 ibc-0 10-grandpa-0 10-grandpa-0 -s connection-0
# ./target/debug/hermes -c config.toml query connection end ibc-1 connection-0
# ./target/debug/hermes -c config.toml query connection end ibc-0 connection-0
sleep 5
echo "########################################################"
echo "hermes -c config.toml tx raw conn-ack ibc-0 ibc-1 10-grandpa-0 10-grandpa-0 -d connection-0 -s connection-0"
echo "########################################################"
hermes -c config.toml tx raw conn-ack ibc-0 ibc-1 10-grandpa-0 10-grandpa-0 -d connection-0 -s connection-0      
sleep 8
echo "########################################################"
echo "hermes -c config.toml tx raw conn-confirm ibc-1 ibc-0 10-grandpa-0 10-grandpa-0 -d connection-0 -s connection-0"
echo "########################################################"
hermes -c config.toml tx raw conn-confirm ibc-1 ibc-0 10-grandpa-0 10-grandpa-0 -d connection-0 -s connection-0  
sleep 5
# ./target/debug/hermes -c config.toml query connection end ibc-0 connection-0  #成功
# ./target/debug/hermes -c config.toml query connection end ibc-1 connection-0

#channle
echo "########################################################"
echo "hermes -c config.toml tx raw chan-open-init ibc-0 ibc-1 connection-0 transfer transfer -o UNORDERED"
echo "########################################################"
hermes -c config.toml tx raw chan-open-init ibc-0 ibc-1 connection-0 transfer transfer -o UNORDERED                  
sleep 5
echo "########################################################"
echo "hermes -c config.toml tx raw chan-open-try ibc-1 ibc-0 connection-0 transfer transfer -s channel-0"
echo "########################################################"
hermes -c config.toml tx raw chan-open-try ibc-1 ibc-0 connection-0 transfer transfer -s channel-0
sleep 5
# ./target/debug/hermes -c config.toml query channel end ibc-1 transfer channel-0
# ./target/debug/hermes -c config.toml query channel end ibc-0 transfer channel-0
echo "########################################################"
echo "hermes -c config.toml tx raw chan-open-ack ibc-0 ibc-1 connection-0 transfer transfer -d channel-0 -s channel-0"
echo "########################################################"
hermes -c config.toml tx raw chan-open-ack ibc-0 ibc-1 connection-0 transfer transfer -d channel-0 -s channel-0
sleep 8
echo "########################################################"
echo "hermes -c config.toml tx raw chan-open-confirm ibc-1 ibc-0 connection-0 transfer transfer -d channel-0 -s channel-0"
echo "########################################################"
hermes -c config.toml tx raw chan-open-confirm ibc-1 ibc-0 connection-0 transfer transfer -d channel-0 -s channel-0
sleep 5
# ./target/debug/hermes -c config.toml tx raw chan-close-init ibc-0 ibc-1 connection-0 transfer transfer -d channel-0 -s channel-0
# ./target/debug/hermes -c config.toml tx raw chan-close-confirm ibc-1 ibc-0 connection-0 transfer transfer -d channel-0 -s channel-0

echo "########################################################"
echo "hermes -c config.toml tx raw ft-transfer ibc-1 ibc-0 transfer channel-0 9999 -o 60 -n 1 -t 9999"
echo "########################################################"
hermes -c config.toml tx raw ft-transfer ibc-1 ibc-0 transfer channel-0 9999 -o 60 -n 1 -t 9999
echo "########################################################"
echo "hermes -c config.toml tx raw packet-recv ibc-1 ibc-0 transfer channel-0"
echo "########################################################"
hermes -c config.toml tx raw packet-recv ibc-1 ibc-0 transfer channel-0
echo "########################################################"
echo "hermes -c config.toml tx raw packet-ack ibc-0 ibc-1 transfer channel-0"
echo "########################################################"
hermes -c config.toml tx raw packet-ack ibc-0 ibc-1 transfer channel-0
