// SPDX-License-Identifier: MIT

use anyhow::Result;
use dbus::blocking::Connection;
use dbus::Message;
use dbus_crossroads::{Crossroads, Context, IfaceBuilder};

const DBUS_NAME: &str = "org.example.rusty";
const DBUS_PATH: &str = "/dbus/rust/service";
const DBUS_INTERFACE: &str = "org.example.rusty";
const DBUS_METHOD_PROCESS_DATA: &str = "ProcessData";
// It's important to use such notation in return messages because of the realization of the underlay library
const DBUS_METHOD_RETURN_SUCCESS: &str = "org.example.rusty.ProcessData.Success";
const DBUS_METHOD_RETURN_FAIL: &str = "org.example.rusty.ProcessData.Failed";
const DBUS_SIGNAL_DATA_PROCESSED: &str = "DataProcessed";

fn main() {
    let start_dbus_service_result = start_dbus_service();
    match start_dbus_service_result {
        Ok(()) => (),
        Err(err) => println!("Error during starting D-Bus service: {err}"),
    }
}

pub fn start_dbus_service() -> Result<()> {
    println!("Starting  D-Bus service...");
    let connection: Connection = Connection::new_system()?;
    connection.request_name(DBUS_NAME, false, true, false)?;
    let mut cr = Crossroads::new();
    let iface_token = cr.register(DBUS_NAME, handle_client_message);
    cr.insert(DBUS_PATH, &[iface_token], ());

    println!("D-Bus service: handling incoming...");
    cr.serve(&connection)?;
    unreachable!()
}

fn handle_client_message(builder: &mut IfaceBuilder<()>) {
    builder.signal::<(String,), _>(DBUS_SIGNAL_DATA_PROCESSED.to_string(), ("ProcessedData",));

    builder.method(
        DBUS_METHOD_PROCESS_DATA,
        ("data",),
        ("ret",),
        move |ctx: &mut Context, _, (data,): (String,)| {
            match do_some_data_processing(data) {
                Ok(processed_data) => {
                    send_processed_data(processed_data, ctx);
                    return Ok((DBUS_METHOD_RETURN_SUCCESS.to_string(),));
                },
                Err(err) =>  {
                    let error : String = format!("Error during data processing: {}", err);
                    eprintln!("{}", error);
                    return Err((DBUS_METHOD_RETURN_FAIL,error).into());
                }
            }
        },
    );
}

fn do_some_data_processing(data: String)  -> Result<String> {
    return Ok(format!("processed: {}", data));
}

fn send_processed_data(processed_data: String, ctx: &mut Context) {
    let msg = Message::signal(
        &DBUS_PATH.into(),
        &DBUS_INTERFACE.into(),
        &DBUS_SIGNAL_DATA_PROCESSED.into());
    ctx.push_msg(msg.append1(processed_data));
}
