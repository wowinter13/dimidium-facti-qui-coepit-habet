#!/bin/bash
# setup.sh
python3 -m venv venv
source venv/bin/activate || source venv/Scripts/activate

# Install requirements
pip3 install -r requirements.txt

# Run the test
python3 test_websocket.py