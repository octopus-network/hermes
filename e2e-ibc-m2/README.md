## Intro
The e2e script is responsible for testing IBC use cases between substrate chains.   
The current test cases are: 
### Establish Client, Connection, and Channel to Connect the 2 Chains 
+ tx raw create-client	
+ tx raw update-client	
+ tx raw conn-init	
+ tx raw conn-try	
+ tx raw conn-ack	
+ tx raw conn-confirm	
+ tx raw chan-open-init	
+ tx raw chan-open-try	
+ tx raw chan-open-ack	
+ tx raw chan-open-confirm
### Packet Transfer
+ tx raw ft-transfer: Send packets from one chain to the other and get acknowlegement. 

### Requirement
```
python3.8+
```
### Install dependencies
```
pip install toml
```
### Start app chain
- Start 2 substrate appchains 
- Update the relayer config file(hermes.toml in the script) based on the 2 chains above if necessary
### Run test script
```
# Get help info
python e2e/run.py -h

    usage: run.py [-h] -c CONFIG_FILE [--cmd CMD] [--log-level LOG]

    Test all relayer commands, end-to-end

    optional arguments:
    -h, --help            show this help message and exit
    -c CONFIG_FILE, --config CONFIG_FILE
                            configuration file for the relayer
    --cmd CMD             command to run the relayer (default: hermes)
    --log-level LOG       minimum log level (default: debug)


# Run test case
python run.py -c ../hermes.toml --cmd hermes

# The output may look like:
hermes -c hermes.toml --json tx raw create-client earth ibc-0

{
  "result": {
    "CreateClient": {
      "client_id": "10-grandpa-38",
      "client_type": "Grandpa",
      "consensus_height": {
        "revision_height": 5192,
        "revision_number": 0
      },
      "height": {
        "revision_height": 82038,
        "revision_number": 0
      }
    }
  },
  "status": "success"
}
...

hermes -c hermes.toml --json tx raw ft-transfer ibc-0 earth transfer channel-36 9999 -o 1000 -d atom -k
alice

{
    "result": [
    {
    "SendPacket":
    {"height": {
          "revision_height": 82158,
          "revision_number": 0
        },
        "packet": {
          "data": "7B22616D6F756E74223A2239393939222C2264656E6F6D223A2261746F6D222C227265636569766572223A2230434441334634374546334334393036363933423137304546363530454239363843354634423243222C2273656E646572223A22636F736D6F73317868326A767A396563747938716463746C6773636D797332647235677A3732396B306C377834227D",
          "destination_channel": "channel-2",
          "destination_port": "transfer",
          "sequence": 1,
          "source_channel": "channel-36",
          "source_port": "transfer",
          "timeout_height": {
            "revision_height": 6313,
            "revision_number": 0
          },
          "timeout_timestamp": {
            "time": null
          }
        }
    }
    }
  ],
  "status": "success"
}
....

```  
