use crate::consts::*;
use crate::gio::glib::Sender;
use gtk::gio;
use gtk::gio::{
    BusNameOwnerFlags, BusNameWatcherFlags, BusType, DBusConnection, DBusMessage,
    DBusMethodInvocation, DBusNodeInfo, DBusSendMessageFlags, OwnerId,
};
use gtk::glib::{MainLoop, Variant, VariantTy};
use std::process::exit;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum CrabDaemonMethod {
    ShowWindow,
}

impl FromStr for CrabDaemonMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ShowWindow" => Ok(Self::ShowWindow),
            _ => Err(()),
        }
    }
}

impl ToString for CrabDaemonMethod {
    fn to_string(&self) -> String {
        match self {
            CrabDaemonMethod::ShowWindow => "ShowWindow",
        }
        .into()
    }
}

pub struct CrabDaemonServer;

impl CrabDaemonServer {
    pub fn new() -> Self {
        Self
    }

    pub fn start(&self, tx: Sender<bool>) -> OwnerId {
        gio::bus_own_name(
            BusType::Session,
            DBUS_SESSION_NAME,
            BusNameOwnerFlags::NONE,
            move |conn, name| Self::on_bus_acquired(conn, name, tx.clone()),
            Self::on_name_acquired,
            Self::on_name_lost,
        )
    }

    fn handle_method_call(
        _connection: DBusConnection,
        _sender: &str,
        _object_path: &str,
        _interface_name: &str,
        method_name: &str,
        _parameters: Variant,
        _invocation: DBusMethodInvocation,
        tx: Sender<bool>,
    ) {
        let method = CrabDaemonMethod::from_str(method_name);

        if let Ok(method) = method {
            match method {
                CrabDaemonMethod::ShowWindow => tx.send(true).unwrap(),
            }
        }
    }

    fn handle_get_property(
        _connection: DBusConnection,
        _sender: &str,
        _object_path: &str,
        _interface_name: &str,
        _property_name: &str,
    ) -> Variant {
        Variant::from_none(VariantTy::ANY)
    }

    fn handle_set_property(
        _connection: DBusConnection,
        _sender: &str,
        _object_path: &str,
        _interface_name: &str,
        _property_name: &str,
        _value: Variant,
    ) -> bool {
        true
    }

    fn on_bus_acquired(connection: DBusConnection, _name: &str, tx: Sender<bool>) {
        let introspection_xml = format!(
            "\
            <node>\
              <interface name='{}'>\
                <method name='ShowWindow'/>\
              </interface>\
            </node>",
            DBUS_INTERFACE_NAME
        );

        let introspection_data = DBusNodeInfo::for_xml(&introspection_xml).unwrap();

        let _registration_id = connection
            .register_object(
                DBUS_OBJECT_PATH,
                &introspection_data
                    .lookup_interface(DBUS_INTERFACE_NAME)
                    .unwrap(),
                move |connection,
                      sender,
                      object_path,
                      interface_name,
                      method_name,
                      parameters,
                      invocation| {
                    Self::handle_method_call(
                        connection,
                        sender,
                        object_path,
                        interface_name,
                        method_name,
                        parameters,
                        invocation,
                        tx.clone(),
                    )
                },
                Self::handle_get_property,
                Self::handle_set_property,
            )
            .unwrap();
    }

    fn on_name_acquired(_connection: DBusConnection, _name: &str) {}

    fn on_name_lost(_connection: Option<DBusConnection>, _name: &str) {
        exit(1);
    }
}

pub struct CrabDaemonClient;

impl CrabDaemonClient {
    pub fn new() -> Self {
        Self
    }

    pub fn run_method(&self, method: CrabDaemonMethod) {
        let watcher_id = gio::bus_watch_name(
            BusType::Session,
            DBUS_SESSION_NAME,
            BusNameWatcherFlags::NONE,
            move |connection, name, name_owner| {
                Self::on_name_appeared(connection, name, name_owner, method);
            },
            Self::on_name_vanished,
        );

        MainLoop::new(None, false).run();

        gio::bus_unwatch_name(watcher_id);
    }

    fn on_name_appeared(
        connection: DBusConnection,
        _name: &str,
        name_owner: &str,
        method: CrabDaemonMethod,
    ) {
        let method_call_message = DBusMessage::new_method_call(
            Some(name_owner),
            DBUS_OBJECT_PATH,
            Some(DBUS_INTERFACE_NAME),
            &method.to_string(),
        );

        connection
            .send_message(&method_call_message, DBusSendMessageFlags::NONE)
            .unwrap();

        exit(0);
    }

    fn on_name_vanished(_connection: DBusConnection, _name: &str) {
        exit(1);
    }
}
