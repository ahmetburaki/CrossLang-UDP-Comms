use std::net::UdpSocket;

fn main() {
    let server_address = "192.168.64.4:8080"; // server ip and port 
    let message = "Hello, UDP Server!";
    
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    
    // Send data to the server
    socket.send_to(message.as_bytes(), server_address).expect("Failed to send data");
    
    println!("Sent: {}", message);
}
