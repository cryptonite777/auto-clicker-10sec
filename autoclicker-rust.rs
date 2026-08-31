use enigo::{Button, Direction, Enigo, Settings};
use std::thread;
use std::time::Duration;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    AtomEnum, ConnectionExt, GetPropertyReply,
};

fn minecraft_is_active() -> bool {
    let (conn, screen_num) = match x11rb::connect(None) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let root = conn.setup().roots[screen_num].root;

    let net_active_window = match conn
    .intern_atom(false, b"_NET_ACTIVE_WINDOW")
    {
        Ok(cookie) => match cookie.reply() {
            Ok(reply) => reply.atom,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let window = match conn
    .get_property(
        false,
        root,
        net_active_window,
        AtomEnum::WINDOW,
        0,
        1,
    )
    .and_then(|cookie| cookie.reply())
    {
        Ok(reply) => {
            if reply.value_len == 0 {
                return false;
            }

            match reply.value32() {
                Some(mut values) => match values.next() {
                    Some(w) => w,
                    None => return false,
                },
                None => return false,
            }
        }
        Err(_) => return false,
    };

    let wm_name = match conn.intern_atom(false, b"_NET_WM_NAME") {
        Ok(cookie) => match cookie.reply() {
            Ok(reply) => reply.atom,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let utf8_string = match conn.intern_atom(false, b"UTF8_STRING") {
        Ok(cookie) => match cookie.reply() {
            Ok(reply) => reply.atom,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let property: GetPropertyReply = match conn
    .get_property(
        false,
        window,
        wm_name,
        utf8_string,
        0,
        1024,
    )
    .and_then(|cookie| cookie.reply())
    {
        Ok(reply) => reply,
        Err(_) => return false,
    };

    let title = String::from_utf8_lossy(&property.value);

    title.to_lowercase().contains("minecraft")
}

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    println!("Auto-clicker running.");
    println!("Only clicks when Minecraft is the active window.");
    println!("Ctrl+C to stop.");

    loop {
        if minecraft_is_active() {
            enigo.button(Button::Left, Direction::Press).unwrap();
            enigo.button(Button::Left, Direction::Release).unwrap();

            println!("Clicked!");
        }

        thread::sleep(Duration::from_secs(10));
    }
}
