
extern crate modbus_iiot;

use modbus_iiot::tcp::master::*;
use modbus_iiot::core::ethernet::EthernetMaster;

fn main()
{
    println!("Hello, Modbus world!");
    let mut client = TcpClient::new("192.168.1.74");
    client.connect();
    reading_coils(&mut client);
    reading_holding_registers(&mut client);
    reading_input_registers(&mut client);
    reading_discrete_inputs(&mut client);
    client.disconnect();
}

fn reading_coils(client: &mut TcpClient) {

    let response = client.read_coils(0, 1);
    println!("Response CO: {:?}", response);

    let response = client.read_coils(0, 10);
    println!("Response CO: {:?}", response);
}

fn reading_holding_registers(client: &mut TcpClient) {

    let response = client.read_holding_registers(0, 1);
    println!("Response HR: {:?}", response);

    let response = client.read_holding_registers(0, 10);
    println!("Response HR: {:?}", response);
}

fn reading_input_registers(client: &mut TcpClient) {

    let response = client.read_input_registers(0, 1);
    println!("Response IR: {:?}", response);

    let response = client.read_input_registers(0, 10);
    println!("Response IR: {:?}", response);
}

fn reading_discrete_inputs(client: &mut TcpClient) {
    
    let response = client.read_discrete_inputs(0, 1);
    println!("Response DI: {:?}", response);   

    let response = client.read_discrete_inputs(0, 10);
    println!("Response DI: {:?}", response);   
}