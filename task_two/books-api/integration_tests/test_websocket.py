import asyncio
import json
import websockets
import sys
from datetime import datetime

async def test_book_flow():
    uri = "ws://localhost:8080/ws"
    try:
        async with websockets.connect(uri) as websocket:
            print(f"[{datetime.now()}] Connected to WebSocket server")

            add_command = {
                "action": "add_book",
                    "book": {
                        "title": "The Rust Programming Language",
                        "author": "Steve Klabnik",
                        "year": 2023
                    }
            }
            print(f"\n[{datetime.now()}] Sending add book command:")
            print(json.dumps(add_command, indent=2))
            
            await websocket.send(json.dumps(add_command))
            response = json.loads(await websocket.recv())
            print(f"Response received:")
            print(json.dumps(response, indent=2))
            
            assert response["type"] == "Success", "Failed to add book"
            book_id = response["data"]
            
            get_command = {
                "action": "get_book",
                    "id": book_id
            }
            print(f"\n[{datetime.now()}] Sending get book command:")
            print(json.dumps(get_command, indent=2))
            
            await websocket.send(json.dumps(get_command))
            response = json.loads(await websocket.recv())
            print(f"Response received:")
            print(json.dumps(response, indent=2))
            
            assert response["type"] == "Success", "Failed to get book"
            assert response["data"]["title"] == "The Rust Programming Language"
            
            # Test updating the book
            update_command = {
                "action": "update_book",
                    "id": book_id,
                    "book": {
                        "title": "The Rust Programming Language - Second Edition",
                        "author": "Steve Klabnik",
                        "year": 2024
                    }
            }
            print(f"\n[{datetime.now()}] Sending update book command:")
            print(json.dumps(update_command, indent=2))
            
            await websocket.send(json.dumps(update_command))
            response = json.loads(await websocket.recv())
            print(f"Response received:")
            print(json.dumps(response, indent=2))
            
            assert response["type"] == "Success", "Failed to update book"
            

            print(f"\n[{datetime.now()}] Verifying update:")
            await websocket.send(json.dumps(get_command))
            response = json.loads(await websocket.recv())
            print(json.dumps(response, indent=2))
            
            assert response["type"] == "Success", "Failed to get updated book"
            assert response["data"]["title"] == "The Rust Programming Language - Second Edition"
            
            delete_command = {
                "action": "delete_book",
                    "id": book_id
            }
            print(f"\n[{datetime.now()}] Sending delete book command:")
            print(json.dumps(delete_command, indent=2))
            
            await websocket.send(json.dumps(delete_command))
            response = json.loads(await websocket.recv())
            print(f"Response received:")
            print(json.dumps(response, indent=2))
            
            assert response["type"] == "Success", "Failed to delete book"
            
            print(f"\n[{datetime.now()}] Verifying deletion:")
            await websocket.send(json.dumps(get_command))
            response = json.loads(await websocket.recv())
            print(json.dumps(response, indent=2))
            
            assert response["type"] == "Error", "Book still exists after deletion"
            
            print(f"\n[{datetime.now()}] All tests passed successfully")

    except websockets.exceptions.ConnectionClosedError:
        print(f"\n ERROR: Could not connect to WebSocket server at {uri}")
        print("Make sure the server is running!")
        sys.exit(1)
    except AssertionError as e:
        print(f"\n TEST FAILED: {str(e)}")
        sys.exit(1)
    except Exception as e:
        print(f"\n UNEXPECTED ERROR: {str(e)}")
        sys.exit(1)

def main():
    print(f"[{datetime.now()}] Starting WebSocket API tests...")
    asyncio.get_event_loop().run_until_complete(test_book_flow())

if __name__ == "__main__":
    main()