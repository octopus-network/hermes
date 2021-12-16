#!/usr/bin/env python3

import argparse
import logging as l
from time import time
from typing import Tuple
from pathlib import Path
import toml

import e2e.channel as channel
import e2e.client as client
import e2e.connection as connection
import e2e.packet as packet
import e2e.relayer as relayer
from e2e.cmd import Config
from e2e.common import *


def passive_packets(c: Config, ibc0: ChainId, ibc1: ChainId, port_id: PortId,
                    ibc0_channel_id: ChannelId, ibc1_channel_id: ChannelId,
                    key0: KeyName, key1: KeyName):

    #start relaying - it should clear the unreceived packets
    proc = relayer.start(c)
    sleep(40.0)

    #send some packets
    # hermes tx raw ft-transfer ibc-0 ibc-1 transfer channel-1 10000 1000 -n 3
    packet.packet_send(c,
                       src=ibc1,
                       dst=ibc0,
                       src_port=port_id,
                       src_channel=ibc1_channel_id,
                       amount=10000,
                       height_offset=1000,
                       number_msgs=1,
                       key=key1)

    # hermes tx raw ft-transfer ibc-1 ibc-0 transfer channel-0 10000 1000 -n 4
    packet.packet_send(c,
                       src=ibc0,
                       dst=ibc1,
                       src_port=port_id,
                       src_channel=ibc0_channel_id,
                       amount=10000,
                       height_offset=1000,
                       number_msgs=1,
                       key=key0)

    #wait for the relayer to initialize and pick up pending packets
    sleep(120.0)

    # query pending packets
    # hermes query packet unreceived-packets ibc-1 transfer channel-1
    packet.query_unreceived_packets(c,
                                    chain=ibc1,
                                    port=port_id,
                                    channel=ibc1_channel_id)

    # hermes query packet unreceived-acks ibc-1 transfer channel-1
    packet.query_unreceived_acks(c,
                                 chain=ibc1,
                                 port=port_id,
                                 channel=ibc1_channel_id)

    # hermes query packet unreceived-packets ibc-0 transfer channel-0
    packet.query_unreceived_packets(c,
                                    chain=ibc0,
                                    port=port_id,
                                    channel=ibc0_channel_id)

    # hermes query packet unreceived-acks ibc-0 transfer channel-0
    packet.query_unreceived_acks(c,
                                 chain=ibc0,
                                 port=port_id,
                                 channel=ibc0_channel_id)

    sleep(120.0)
    # Stop the relayer
    # proc.kill()


def raw(c: Config, ibc0: ChainId, ibc1: ChainId, ibc0_chan_id: ChannelId,
        ibc1_chan_id: ChannelId, port_id: PortId):

    # Send packet between substrate and cosmos.
    packet.ping_pong(c, ibc0, ibc1, ibc0_chan_id, ibc1_chan_id, port_id)

    # sleep(5)

    # Test send packet time out
    # packet.timeout(c, ibc0, ibc1, ibc0_chan_id, ibc1_chan_id)

    # split()
    # # The ChannelCloseInit message is currently denied by Gaia,
    # # and requires a patch to be accepted.
    # channel.close(c, ibc0 , ibc1, ibc0_conn_id,
    #               ibc1_conn_id, ibc0_chan_id, ibc1_chan_id)


def main():
    parser = argparse.ArgumentParser(
        description='Test the relayer commands, end-to-end')

    parser.add_argument('-c',
                        '--config',
                        help='configuration file for the relayer',
                        metavar='CONFIG_FILE',
                        required=True,
                        type=Path)

    parser.add_argument('--cmd',
                        help='command to run the relayer (default: hermes)',
                        metavar='CMD',
                        default='hermes')

    parser.add_argument(
        '--log-level',
        help='minimum log level (default: debug)',
        metavar='LOG',
        choices=['notset', 'debug', 'info', 'warning', 'error', 'critical'],
        default='debug')

    args = parser.parse_args()

    if not args.config.exists():
        print(
            f'error: supplied configuration file does not exist: {args.config}')
        exit(1)

    config = Config(config_file=args.config,
                    relayer_cmd=args.cmd,
                    log_level=args.log_level.upper())

    l.basicConfig(level=config.log_level,
                  format='%(asctime)s [%(levelname)8s] %(message)s',
                  datefmt='%Y-%m-%d %H:%M:%S')

    hermesConfig = toml.load(config.config_file)

    chains = hermesConfig['chains']

    ibc0 = chains[0]['id']
    key0 = chains[0]['key_name']
    ibc1 = chains[1]['id']
    key1 = chains[1]['key_name']
    port_id = PortId('transfer')

    # count run time
    begin_time = time()

    ibc0_client_id, ibc1_client_id = client.create_update_query_client(config, ibc0, ibc1)
    # ibc1_client_id = client.create_update_query_client(config, ibc1, ibc0)
    # Connection handshake and get connection id
    ibc0_conn_id, ibc1_conn_id = connection.handshake(config, ibc0, ibc1,
                                                      ibc0_client_id,
                                                      ibc1_client_id)
    split()
    # Channel handshake and get channel id
    ibc0_chan_id, ibc1_chan_id = channel.handshake(config, ibc0, ibc1,
                                                   ibc0_conn_id, ibc1_conn_id,
                                                   port_id)

    # Test for  tx raw mode
    # print('\033[32;1m{msg}\033[0m'.format(
    #     msg=".............. Begin to test by raw mode  .............."))
    # sleep(2.0)
    # raw(config, ibc0, ibc1, ibc0_chan_id, ibc1_chan_id, port_id)
    # print('\033[32;1m{msg}\033[0m'.format(
    #     msg=".............. Raw mode end .............."))
    #
    # print('\n\n')

    # Test for passive mode
    print('\033[32;1m{msg}\033[0m'.format(
        msg=".............. Begin to test by passive mode  .............."))
    sleep(2.0)
    passive_packets(config, ibc0, ibc1, port_id, ibc0_chan_id, ibc1_chan_id,
                    key0, key1)
    print('\033[32;1m{msg}\033[0m'.format(
        msg=".............. Passive mode end .............."))
    print()

    end_time = time()
    time_costing = end_time - begin_time

    print('\033[32;1mThe test case execution duration:{msg}\033[0m'.format(
        msg=time_costing))


if __name__ == "__main__":
    main()
