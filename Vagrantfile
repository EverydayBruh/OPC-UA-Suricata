Vagrant.configure("2") do |config|
  config.vm.define "server_vm" do |server_vm|
    server_vm.vm.box = "bento/ubuntu-20.04-arm64"
    server_vm.vm.provider "vmware_desktop"

    server_vm.vm.network "private_network", ip: "192.168.56.10"

    server_vm.vm.provision "shell", inline: <<-SHELL
      sudo apt-get update
      sudo apt-get install -y python3 python3-pip

      mkdir -p /home/vagrant/opcua_example
      cp /vagrant/server_example_python/opcua_server.py /home/vagrant/opcua_example/opcua_server.py

      sudo ufw allow 4840
      pip3 install opcua

      python3 /home/vagrant/opcua_example/opcua_server.py &
      
    SHELL
  end

  config.vm.define "client_vm" do |client_vm|
    client_vm.vm.box = "bento/ubuntu-20.04-arm64"
    client_vm.vm.provider "vmware_desktop"

    client_vm.vm.network "private_network", ip: "192.168.56.11"

    client_vm.vm.provision "shell", inline: <<-SHELL
      sudo apt-get update
      sudo apt-get install -y python3 python3-pip

      mkdir -p /home/vagrant/opcua_example
      cp /vagrant/server_example_python/opcua_client.py /home/vagrant/opcua_example/opcua_client.py
      pip3 install opcua

      sed -i 's/SERVER_IP/192.168.56.10/' /home/vagrant/opcua_example/opcua_client.py

      python3 /home/vagrant/opcua_example/opcua_client.py &
    SHELL
  end
end

