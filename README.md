## Chat application over TCP, create connection via UDP discovery
1. Any user can become host.
2. Host starts broadcasting UDP discovery packets over its network.
3. Other users can act as client and listens on the port.
4. Client get the packets and add ip:port of host to its library.

## Debugging
- Client: Listen on `--ip 0.0.0.0 --port 5500`
- Host: Start broadcasting on `--ip 255.255.255.255 --port 5500`
- Connect to client with cli `socat TCP4:127.0.0.1:4500,sourceport=5500 -`

## TODO
-> HOST: when cmd is `Start` ? it will start listening for tcp connection from clients.
-> CLIENT: when cmd is `Connect <host-ip:host-port>` ? it will connect to host. 