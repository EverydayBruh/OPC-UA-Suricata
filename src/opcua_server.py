from opcua import Server
import time

# Create a server instance
server = Server()

server.set_endpoint("opc.tcp://0.0.0.0:4840/freeopcua/server/")

server.set_server_name("My OPC UA Server")

uri = "http://example.org"
idx = server.register_namespace(uri)

my_obj = server.nodes.objects.add_object(idx, "MyObject")

my_var = my_obj.add_variable(idx, "MyVariable", 101)

my_var.set_writable()

server.start()
print("Server is running...")
print("Nodes in address space:")
for node in server.nodes.objects.get_children():
    print(node)

try:
    while True:
        time.sleep(1)  
except KeyboardInterrupt:
    print("Server is shutting down...")
finally:
    server.stop()
    print("Server stopped.")

