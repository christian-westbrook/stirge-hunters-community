// ----------------------------------------------------------------------------
// System     : Stirge Engine
// Service    : Crucible
// File       : main.rs
// Engineer   : Christian Westbrook
// Copyright  : (c) 2026 Christian Westbrook
// License    : Distributed under the MIT license. See the LICENSE file in the
//              repository root for details.
//
//  Abstract
//! Provides an entrypoint to the Crucible MMO server.
//! 
//! Crucible uses the Tokio async runtime to multiplex client
//! WebSocket connections across threads.
// ----------------------------------------------------------------------------

use tokio::sync::mpsc;

use crucible::networking::listeners::ClientListener;

const CLIENT_LISTENER_ADDR: &str = "0.0.0.0:7070";

// Initializes an HTTP server for listening to inbound client connections.
#[tokio::main]
async fn main() {

    intro();

    // Setup client connection queue
    let (client_conn_queue_tx, client_conn_queue_rx) = mpsc::channel(100);

    // Setup client listener
    let client_listener = ClientListener::new(CLIENT_LISTENER_ADDR, client_conn_queue_tx);

    tokio::join!(
        client_listener.start_listening(),
    );
}

// Prints an intro message to the command-line
fn intro() {
	print!("\n");
	print!("     _,-'\"`-._,-'\"`-._,-'\"`-._\n");
	print!("           STIRGE ENGINE\n");
    print!("        CRUCIBLE MMO SERVER\n");
	print!("     \"`-._,-'\"`-._,-'\"`-._,-'\"\n");
	print!("\n");

	print!("........:@.....:@@@@-.....@:........\n");
	print!(".......@.@@:.+@@@@@@@@+..@@.@:......\n");
	print!("....=*:%..:#-+@@@@@@@@+-%:..%:*=....\n");
	print!("..-*+-..#..*=+#@@@@@@#+=#..#..:+*-..\n");
	print!(".@:.........*@#.@@@@:*@#..........@.\n");
	print!("@....-@@@@.......@@.......@@@@=....@\n");
	print!("@..-+**..@=......@@......=@..**+-..@\n");
	print!("@@@=.......%@+...@@...+@@.......-@@@\n");
	print!("....+#..@@@@=.....@....-@@@@..*+....\n");
	print!("..=+-:+...........+..........+--+=..\n");
	print!(".@:*+..........................+#.@.\n");
	print!("@......@@..%@+...@....+@@..@@:.....@\n");
	print!("@....-@.....*#=.+....=##.....%=....@\n");
	print!(".@:.+#.......+#..@...*+.......*+..@.\n");
	print!(".@:*+.........-@@@@@@=.........+#.@.\n");
	print!("..-:............=@@=............:=..\n");
	print!(".................@@.................\n");
	print!("............*+..@@@.................\n");
	print!(".............:=@@-..................\n");

	print!("\n");
	print!("A graphical MUD engine by Christian Westbrook\n");
	print!("\n");
}
