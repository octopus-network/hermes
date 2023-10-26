# Misbehaviour Testing for cosmos -> near


## Step1

Clone verification-proxy repo and setup.

```bash
$ git clone https://github.com/octopus-network/verification-proxies.git
$ cd verification-proxies
$ ./scripts/local.sh
```

## Step2

Clone octopus gaia repo and make install

```bash
$ git clone https://github.com/octopus-network/gaia.git
$ cd gaia
$ git checkout feat/sdk-47-ibc-7
$ make install
```

## Step3

Clone octopus hermes repo and cargo build

```bash
$ git clone https://github.com/octopus-network/hermes.git
$ cd hermes
$ git checkout v1.7.0-octopus
$ cargo build -p ibc-relayer-cli
```

## Step4

Run misbehaviour test script and watch test result

```bash
$ cd hermes/ci/misbehaviour
$ cd hermes
$ ./misbehaviour_test.sh
```