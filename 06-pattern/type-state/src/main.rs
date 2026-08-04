
fn main() {
    let rc = RemoteController::new();
    let mut rc = rc.play().stop();
    rc.mute();
    rc.unmute();
    rc.see_details();
    rc.power_off();
}

// 1. States (Nouns / Adjectives)
struct Idle;
struct Playing;
struct Stopped;

trait Inspectable {}
impl Inspectable for Playing {}
impl Inspectable for Stopped {}

struct RemoteController<State> {
    audio: usize,
    s: State,
}

impl <S> RemoteController<S> {
    fn power_off(self) {
        println!("Power off!");
    }

    fn mute(&mut self) {
        println!("Audio muted: 0");
        self.audio = 0;

    }

    fn unmute(&mut self) {
        self.audio = 50;
        println!("Audio unmuted: 50");
    }
}

impl<S: Inspectable> RemoteController<S> {
    fn see_details(&self) {
        println!("Displaying state diagnostic info...");
    }
}

impl RemoteController<Idle> {
    fn new() -> Self {
        RemoteController { s: Idle, audio: 50 }
    }

    fn play(self) -> RemoteController<Playing> {
        println!("Started playing.");
        RemoteController { s: Playing, audio: self.audio }
    }
}

impl RemoteController<Playing> {

    fn stop(self) -> RemoteController<Stopped> {
        println!("Stopped playback.");
        RemoteController { s: Stopped, audio: self.audio }
    }
}

impl RemoteController<Stopped> {
    fn play(self) -> RemoteController<Playing> {
        println!("Restarting playback.");
        RemoteController { s: Playing, audio: self.audio }
    }
}