from opcua import Client
import time

client = Client("opc.tcp://192.168.56.10:4840/freeopcua/server/")

try:
    client.connect()
    print("Client is connected to the server.")

    obj_node = client.get_node("ns=2;i=1")

    var = obj_node.get_child("2:MyVariable")  # `2` is the namespace index for "MyVariable"

    while True:
        value = var.get_value()
        print(f"Current value: {value}")
        var.set_value(value + 5)  
        time.sleep(1)

except Exception as e:
    print(f"An error occurred: {e}")

finally:
    client.disconnect()
    print("Client disconnected.")

