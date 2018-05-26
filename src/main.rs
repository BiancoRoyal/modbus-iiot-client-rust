
extern crate modbus_iiot;

use modbus_iiot::tcp::master::TcpClient;
use modbus_iiot::tcp::masteraccess::{MasterAccess, CoilValue};

fn main()
{
    let mut client = TcpClient::new("[::1]:10502");
    let result = client.connect();
    match result {
        Err(message) => println!("Ups, No Modbus world! {}", message),
        Ok(_) => {
            println!("Hello, Modbus world!");
            reading_coils(&mut client);
            reading_holding_registers(&mut client);
            reading_input_registers(&mut client);
            reading_discrete_inputs(&mut client);

            write_coils(&mut client);
            write_registers(&mut client);

            client.disconnect();
        }
    }
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

// =========================================================================

fn write_coils(client: &mut TcpClient) {
    
    let response = client.write_single_coil(1, CoilValue::On);
    println!("Response WSCO: {:?}", response);   

    let response = client.write_multiple_coils(0, vec!(CoilValue::On, CoilValue::Off, CoilValue::On));
    println!("Response WMCO: {:?}", response);   
}

fn write_registers(client: &mut TcpClient) {
    
    let response = client.write_single_register(1, 65000);
    println!("Response WSRE: {:?}", response);   

    let response = client.write_multiple_registers(0, vec!(23456, 77, 65534, 0));
    println!("Response WMRE: {:?}", response);   
}