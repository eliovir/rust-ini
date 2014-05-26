#![crate_id="design_pattern-command#1.0"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-0.11-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-20

//! Each action is encapsulated into a struct with the trait Command
//! where only the method `execute()` is run.
trait Command {
	fn execute(&self);
}

// Use a Null struct to initialize the remote control.
struct NullCommand;
impl NullCommand {
	fn new() -> NullCommand {
		NullCommand
	}
}
impl Command for NullCommand {
	fn execute(&self) {
		println!("Nothing to do!");
	}
}

// The object to handle: a light
struct Light;
impl Light {
	fn new() -> Light {
		Light
	}
	fn on(&self) {
		println!("Light is on");
	}
	fn off(&self) {
		println!("Light is off");
	}
}

// The first command on the object: light on
struct LightOnCommand {
	light: Light,
}
impl LightOnCommand {
	fn new(light: Light) -> LightOnCommand {
		LightOnCommand { light: light }
	}
}
impl Command for LightOnCommand {
	fn execute(&self) {
		self.light.on();
	}
}

// The second command on the object: light off
struct LightOffCommand {
	light: Light,
}
impl LightOffCommand {
	fn new(light: Light) -> LightOffCommand {
		LightOffCommand { light: light }
	}
}
impl Command for LightOffCommand {
	fn execute(&self) {
		self.light.off();
	}
}

// The command will be launched by a remote control.
struct SimpleRemoteControl {
	command: Box<Command>,
}
impl SimpleRemoteControl {
	fn new() -> SimpleRemoteControl {
		SimpleRemoteControl { command: box NullCommand::new() }
	}
	fn setCommand(&mut self, cmd: Box<Command>) {
		self.command = cmd;
	}
	fn buttonWasPressed(&self) {
		self.command.execute();
	}
}

fn main() {
	let mut remote = SimpleRemoteControl::new();
	let light = Light::new();
	let lightOn = LightOnCommand::new(light);
	let lightOff = LightOffCommand::new(light);

	remote.buttonWasPressed();
	remote.setCommand(box lightOn);
	remote.buttonWasPressed();
	remote.setCommand(box lightOff);
	remote.buttonWasPressed();
}