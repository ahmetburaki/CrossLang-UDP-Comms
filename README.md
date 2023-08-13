# CrossLang-UDP-Comms

This repository contains example code for a UDP client written in Rust and a UDP server written in C. These sample codes demonstrate simple communication between the two languages using UDP. They can be helpful for developers looking to understand UDP-based communication.

**C UDP Server Example**

This repository contains a basic C program that demonstrates a UDP server's functionality. The server is designed to receive messages from UDP clients and display them along with client information.

**Functionality:**
The C program creates a UDP socket, binds it to a specified port (default: 8080, you can change it), and then listens for incoming messages from clients. It utilizes various standard libraries such as `<stdio.h>`, `<stdlib.h>`, `<string.h>`, `<unistd.h>`, and `<arpa/inet.h>` to handle socket communication.

**Usage:**
1. Compile the C code: `gcc udp_server.c -o udp_server`
2. Run the compiled executable: `./udp_server`

**Output:**
- The server will start listening on the specified port.
- As clients send messages, the server will display the received message along with the client's IP address and port number.

**Example Output:**
```
UDP Server listening on port 8080...
Received from 192.168.64.1:12345: Hello, UDP Server!
Received from 192.168.64.1:54321: Hello, UDP Server!
```

**Rust UDP Client Example**

This repository contains a Rust program illustrating a UDP client's functionality. The client is designed to send a predefined message to a specified server address and port.

**Functionality:**
The Rust program uses the `std::net::UdpSocket` module to create a UDP socket. It sends a predefined message to the specified server address, which in this case is "192.168.64.4:8080".

**Usage:**
1. Ensure Rust is installed. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. Compile and run the Rust code: `cargo run`

**Output:**
- The client will send the predefined message to the server.
- The sent message will be displayed on the console.

**Example Output:**
```
Sent: Hello, UDP Server!
```

**Note:** For your convenience in observing network activities, I've also included a packet capture (pcap) recording in the repository.