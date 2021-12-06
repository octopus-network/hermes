echo "Create Client earth ibc-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw create-client earth ibc-0            #success 

echo "Query Client state Earth 10-grandpa-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query client state earth 10-grandpa-0       #success

echo "Create client ibc-0 earth"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw create-client ibc-0 earth            #success

echo "Query Client state ibc-0 07-tendemint-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query client state ibc-0  07-tendermint-0   #success

#updateclient

echo "Update client earthg 10-grandpa-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw update-client earth 10-grandpa-0     #success

echo "Query client state earth 10-grandpa-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query client state earth 10-grandpa-0       #success

echo "Update-client ibc-0 07-tendermint-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw update-client ibc-0 07-tendermint-0  #success

echo "Query client state ibc-0 07-terdermint-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query client state ibc-0  07-tendermint-0   #success

#connection
echo "Conn-Init earth ib-0"

#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-init earth ibc-0 10-grandpa-0 07-tendermint-0                                     #success

echo "Conn-Try ibc-0 earth"

#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-try ibc-0 earth 07-tendermint-0 10-grandpa-0 -s connection-0                      #success

echo "Conn-Ack earth ibc-0"

#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-ack earth ibc-0 10-grandpa-0 07-tendermint-0 -d connection-0 -s connection-0      #success

echo "Conn-confirm ibc-0 earth"

#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-confirm ibc-0 earth 07-tendermint-0 10-grandpa-0 -d connection-0 -s connection-0  #success

echo "Query connection end earth"

#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query connection end earth connection-0                                                       #success

echo "Qeury connnection end ibc-0"
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query connection end ibc-0 connection-0                                                       #success


echo "Conn-init ibc-0 earth"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-init  ibc-0 earth  07-tendermint-0 10-grandpa-0                                      #success

echo "Conn-Try earth ibc-0"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-try earth  ibc-0 10-grandpa-0 07-tendermint-0  -s connection-0                      #success

echo "Conn-Ack ibc-0 earth"

./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-ack ibc-0 earth  07-tendermint-0  10-grandpa-0 -d connection-0 -s connection-0      #失败

echo "Conn-confirm earth ibc-0"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw conn-confirm earth  ibc-0 10-grandpa-0 07-tendermint-0  -d connection-0 -s connection-0  #失败

echo "Query connection end ibc-0"

/target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query connection end ibc-0 connection-0  #成功

echo "Query connection end earth"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query connection end earth connection-0  #成功

#channle

echo "Chann-open-init earth ibc-0"
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-init earth ibc-0 connection-0 transfer transfer -o UNORDERED                   #成功

echo "Chan-open-try ibc-0 earth"
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-try ibc-0 earth connection-0 transfer transfer -s channel-0                    #成功

echo "Chan-open-ack earth ibc-0"
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-ack earth ibc-0 connection-0 transfer transfer -d channel-0 -s channel-0       #失败，原因待查

echo "Chan-open-confirm ibc-0 earth"
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-confirm ibc-0 earth connection-0 transfer transfer -d channel-0 -s channel-0   #失败，链接丢失，前面的通道未成功建立导致的

echo "Query channel end earth "
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query channel end earth transfer channel-0                                                      #成功

echo "Query channel end ibc-0"
#./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query channel end ibc-0 transfer channel-0                                                      #失败


echo "Chan-open-init ibc-0 earth"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-init ibc-0 earth  connection-0 transfer transfer -o UNORDERED                  #成功

echo "Chan-open-try earth ibc-0"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-try earth  ibc-0 connection-0 transfer transfer -s channel-0                   #成功

echo "Chan-open-ack ibc-0 earth"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-ack ibc-0 earth connection-0 transfer transfer -d channel-0 -s channel-0       #失败，原因待查

echo "Chan-open-confirm earth ibc-0"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml tx raw chan-open-confirm earth ibc-0 connection-0 transfer transfer -d channel-0 -s channel-0   #失败，原因待查

echo "Query channel end ibc-0 "
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query channel end ibc-0 transfer channel-0  #失败

echo "Query channel end earth"
./target/debug/hermes -c ~/workspace/mock-grandpa/hermes.toml query channel end earth transfer channel-0  #成功1
