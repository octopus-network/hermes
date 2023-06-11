#!/usr/bin/env python3

import argparse
import logging as l
from pathlib import Path
import numbers
import json
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

# import logging
# logging.basicConfig(level=logging.DEBUG)

substrate = SubstrateInterface(
    url="ws://127.0.0.1:9944"
)

# keypair = Keypair.create_from_uri('//Alice')


def creat_asset(asset_id):
    call = substrate.compose_call(
        call_module='Assets',
        call_function='force_create',
        call_params={
            'id': asset_id,
            'owner': '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
            'is_sufficient': True,
            'min_balance': 10,
        }
    )
    # Wrap it in sudo.
    call = substrate.compose_call(
        call_module='Sudo',
        call_function='sudo',
        call_params={
            'call': call.value,
        }
    )

    extrinsic = substrate.create_signed_extrinsic(
        call=call,
        keypair=keypair,
        era={'period': 64}
    )

    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        print('Extrinsic "{}" included in block "{}"'.format(
            receipt.extrinsic_hash, receipt.block_hash
        ))

        if receipt.is_success:

            print('✅ Success, triggered events:')
            for event in receipt.triggered_events:
                print(f'* {event.value}')

        else:
            print('⚠️ Extrinsic Failed: ', receipt.error_message)

    except SubstrateRequestException as e:
        print("Failed to send: {}".format(e))


def query_asset_info():
    result = substrate.query_map(
        module='Ics20Transfer',
        storage_function='AssetIdByName',
        # params=[666,'5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty']
    )
    for record in result:
        print('query_asset_info: ', record)


def query_denom_trace():
    result = substrate.query_map(
        module='Ics20Transfer',
        storage_function='DenomTrace',
        # params=[666,'5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty']
    )
    for record in result:
        print('query_denom_trace: ', record)


def query_asset():
    result = substrate.query(
        module='Assets',
        storage_function='Account',
        params=[666,'5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty']
    )
    print('query_asset: ', result)


def query_account():
    result = substrate.query(
        module='System',
        storage_function='Account',
        params=['5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty']
    )
    print('query_account: ', result)


def tx(args):
    print("tx args",args)
    params = json.loads(args.params)
    print(params)

    call = substrate.compose_call(
        call_module=args.module,
        call_function=args.method,
        call_params=params
    )
    print(call)
    keypair = Keypair.create_from_uri(args.signer)
    # # Wrap it in sudo.
    if args.sudo is not None:
        keypair = Keypair.create_from_uri(args.sudo)
        call = substrate.compose_call(
            call_module='Sudo',
            call_function='sudo',
            call_params={'call': call.value})

    extrinsic = substrate.create_signed_extrinsic(
        call=call,
        keypair=keypair,
        era={'period': 64}
    )

    try:
        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        print('Extrinsic "{}" included in block "{}"'.format(
            receipt.extrinsic_hash, receipt.block_hash
        ))

        if receipt.is_success:

            print('✅ Success, triggered events:')
            for event in receipt.triggered_events:
                print(f'* {event.value}')

        else:
            print('⚠️ Extrinsic Failed: ', receipt.error_message)

    except SubstrateRequestException as e:
        print("Failed to send: {}".format(e))


def query(args):
    print("query args",args)
    # params = list(args.params)
    if args.params is not None:
        params = []
        for param in args.params:
            try:
                params.append(int(param))
            except ValueError:
                params.append(param)
        print(params)
    
        result = substrate.query(
            module=args.module,
            storage_function=args.storage,
            params=params
        )
        print('query result: ', result)

    else:
        result = substrate.query_map(
            module=args.module,
            storage_function=args.storage)
        for record in result:
        #   print('query record: ', record[0],record[1])
          print('query record: ', record)
       
def query_balances(args):
    print("query balances args",args)
    asset_info = [args.asset[0],args.account[0]]
    # for asset in asset_infos:
    print(asset_info)
    result = substrate.query(
        module='System',
        storage_function='Account',
        params=args.account
    )
    print('System Account: ', result)

    # asset_info = zip(args.asset,args.account)
    # asset_info = [args.asset[0],args.account[0]]
    # # for asset in asset_infos:
    # print(asset_info)
    result = substrate.query(
        module='Assets',
        storage_function='Account',
        params=asset_info
        )
    print('query_asset: ', result)
    print(substrate.properties)


def main():
    parser = argparse.ArgumentParser(
        prog='sub_cli',
        description='The sub_cli is a utility used to interact with substrate.')

    parser.add_argument('--url',
                        help=' The substrate endpoint to connect to, e.g. wss://rpc.polkadot.io (default: %(default)s)',
                        # action='store_const',
                        # const=str,
                        # nargs='?',
                        type=str,
                        metavar='url',
                        default='ws://127.0.0.1:9944')
    
    parser.add_argument('--sudo',
                        # action='store_true',
                        help='Execute tx as a sudo account, e.g. "//Alice" or "bottom drive obey lake curtain smoke basket hold race lonely fit walk"',
                        # metavar='sudo'
                        # default=False
                        metavar='account',
                        # nargs='?',
                        type=str,
                        # default='//Alice'
                        )
    # parser.add_argument('--log-level',
    #                     help='minimum log level (default: debug)',
    #                     metavar='LOG',
    #                     choices=['notset', 'debug', 'info',
    #                              'warning', 'error', 'critical'],
    #                     default='debug')
    parser.add_argument('--version', action='version', version='%(prog)s 0.1')
    
    subparsers = parser.add_subparsers(help='Supported sub command: tx|query')

    # add sub command for tx
    parser_tx = subparsers.add_parser('tx',
                                        help='Create and execute a extrinsic transaction.',
                                       description='''  sub_cli tx --module Balances --method transfer --params '{"dest": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","value": 20000000000000000}' ''',
                                       formatter_class=argparse.RawDescriptionHelpFormatter)
    parser_tx.add_argument('--signer',
                        help='''The tx signer, e.g. '//Alice' or 'bottom drive obey lake curtain smoke basket hold race lonely fit walk' (default: '//Alice')''',
                        # nargs=1,
                        type=str,
                        # metavar='module name',
                        # required=True
                        default='//Alice'
                        )
 
    parser_tx.add_argument('--module',
                        help='The module name, e.g. --module Balances',
                        # nargs=1,
                        type=str,
                        # metavar='module name',
                        # required=True
                        )
    parser_tx.add_argument('--method',
                        help='The method name, e.g. --method transfer or --method force_transfer',
                        # nargs=1,
                        type=str,
                        # metavar='Method name',
                        # required=True
                        )
    parser_tx.add_argument('--params',
                        help='''The method params,a json string, e.g. '{"dest": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty","value": 20000000000000000}' ''',
                        # nargs=1,
                        type=str,
            
                        # metavar='Method name',
                        # required=True
                        )
    parser_tx.set_defaults(func=tx)

     # add sub command for query
    parser_query = subparsers.add_parser('query',
                                        help='Query the chain state value for module/pallet storage object.',
                                       description='''  sub_cli query --module System --storage Account --params 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty ''',
                                       formatter_class=argparse.RawDescriptionHelpFormatter)

    parser_query.add_argument('--module',
                        help='The module name, e.g. --module System',
                        # nargs=1,
                        type=str,
                        # metavar='module name',
                        # required=True
                        )
    parser_query.add_argument('--storage',
                        help='The storage name, e.g. --storage Account or --storage BlockHash',
                        # nargs=1,
                        type=str,
                        # metavar='Method name',
                        # required=True
                        )
    parser_query.add_argument('--params',
                        help='''The storage params, e.g. --params 666 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty ''',
                        nargs="*",
                        # type=str,
            
                        # metavar='Method name',
                        # required=True
                        )
    parser_query.set_defaults(func=query)


    # parser.add_argument('--query',
    #                     help='query the state value for module/pallet storage object ,e.g. System Account',
    #                     metavar='query',
    #                     # required=True
    #                     )
 
    parser_query_balances = subparsers.add_parser('query-balances',
                                       help='Query all the asset balances for account.',
                                       description='''  sub_cli query-balances --asset 666  --account 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty''',
                                       formatter_class=argparse.RawDescriptionHelpFormatter)
    parser_query_balances.add_argument('--account',
                        help='''The account, e.g. --account 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty''',
                        nargs="*",
                        # type=str,
            
                        # metavar='Method name',
                        # required=True
                        )
    parser_query_balances.add_argument('--asset',
                        help='''The asset id, e.g. --asset 666''',
                        nargs="*",
                        type=int,
            
                        # metavar='Method name',
                        # required=True
                        )
    parser_query_balances.set_defaults(func=query_balances)
    
    args = parser.parse_args()
    print(args)
    args.func(args)
    # aDict = json.loads(args.params)
    # print(aDict)

   
     

if __name__ == "__main__":
    main()
    # creat_asset(888)
    # query_account()
    # query_asset_info()
    # query_denom_trace()
    # json_str = '{"dest":"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "value": 20000000000000000}'
    # # json_str = '{"a":54, "b": 28}'
    # aDict = json.loads(json_str)
    # print(aDict)