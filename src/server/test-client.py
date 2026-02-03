#!/usr/bin/env python3
"""
Simple Python WebSocket test client for Rustscape
Usage: python test-client.py [username]
"""

import asyncio
import json
import sys
from datetime import datetime

import websockets


async def test_client(username="TestPlayer"):
    uri = "ws://localhost:8080/ws"

    print(f"🎮 Rustscape Test Client")
    print(f"Connecting to {uri}...")

    try:
        async with websockets.connect(uri) as websocket:
            print("✅ Connected!")

            # Receive welcome message
            welcome = await websocket.recv()
            print(f"📨 Received: {welcome}")

            # Login
            login_packet = {
                "type": "Login",
                "username": username,
                "password": "password123",
            }
            await websocket.send(json.dumps(login_packet))
            print(f"📤 Sent login: {username}")

            # Wait for login response
            response = await websocket.recv()
            login_data = json.loads(response)
            print(f"📨 Login response: {json.dumps(login_data, indent=2)}")

            if login_data.get("type") == "LoginSuccess":
                player_id = login_data.get("player_id")
                position = login_data.get("position")
                print(
                    f"✅ Login successful! Player ID: {player_id}, Position: {position}"
                )

                # Send a chat message
                await asyncio.sleep(1)
                chat_packet = {"type": "Chat", "message": f"Hello from {username}!"}
                await websocket.send(json.dumps(chat_packet))
                print(f"📤 Sent chat message")

                # Request players
                await asyncio.sleep(1)
                await websocket.send(json.dumps({"type": "RequestPlayers"}))
                print(f"📤 Requested player list")

                players_response = await websocket.recv()
                players_data = json.loads(players_response)
                print(f"📨 Players: {json.dumps(players_data, indent=2)}")

                # Request NPCs
                await asyncio.sleep(1)
                await websocket.send(json.dumps({"type": "RequestNpcs"}))
                print(f"📤 Requested NPC list")

                npcs_response = await websocket.recv()
                npcs_data = json.loads(npcs_response)
                print(f"📨 NPCs: {json.dumps(npcs_data, indent=2)}")

                # Test movement
                await asyncio.sleep(1)
                new_x = position["x"] + 5
                new_y = position["y"] + 3
                move_packet = {"type": "Move", "x": new_x, "y": new_y}
                await websocket.send(json.dumps(move_packet))
                print(f"📤 Moved to ({new_x}, {new_y})")

                # Ping test
                await asyncio.sleep(1)
                timestamp = int(datetime.now().timestamp() * 1000)
                ping_packet = {"type": "Ping", "timestamp": timestamp}
                await websocket.send(json.dumps(ping_packet))
                print(f"📤 Sent ping")

                pong_response = await websocket.recv()
                pong_data = json.loads(pong_response)
                if pong_data.get("type") == "Pong":
                    latency = (
                        int(datetime.now().timestamp() * 1000) - pong_data["timestamp"]
                    )
                    print(
                        f"📨 Pong! Latency: {latency}ms, Server tick: {pong_data['server_tick']}"
                    )

                # Keep connection alive and listen for broadcasts
                print(f"\n👂 Listening for messages (press Ctrl+C to exit)...")
                try:
                    while True:
                        message = await asyncio.wait_for(websocket.recv(), timeout=30.0)
                        data = json.loads(message)
                        msg_type = data.get("type")

                        if msg_type == "ChatMessage":
                            print(f"💬 [{data['username']}]: {data['message']}")
                        elif msg_type == "PlayerJoined":
                            print(
                                f"👋 Player joined: {data['username']} (ID: {data['id']})"
                            )
                        elif msg_type == "PlayerLeft":
                            print(f"👋 Player left: ID {data['id']}")
                        elif msg_type == "PlayerMoved":
                            print(f"🚶 Player {data['id']} moved to {data['position']}")
                        else:
                            print(f"📨 {json.dumps(data, indent=2)}")

                except asyncio.TimeoutError:
                    print("⏱️  No messages for 30s, sending ping...")
                    await websocket.send(
                        json.dumps(
                            {
                                "type": "Ping",
                                "timestamp": int(datetime.now().timestamp() * 1000),
                            }
                        )
                    )

            else:
                print(f"❌ Login failed: {login_data}")

    except websockets.exceptions.ConnectionClosed:
        print("🔌 Connection closed")
    except KeyboardInterrupt:
        print("\n👋 Disconnecting...")
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback

        traceback.print_exc()


if __name__ == "__main__":
    username = sys.argv[1] if len(sys.argv) > 1 else "TestPlayer"
    asyncio.run(test_client(username))
