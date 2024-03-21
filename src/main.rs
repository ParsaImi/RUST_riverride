use std::io::{stdout, Write, Stdout};
use std::{time::Duration, io};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    QueueableCommand,
    event,
    terminal::size,
    cursor::MoveTo,

};

use crossterm::event::{poll, read, Event};
use crossterm::event::KeyCode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

struct World{
    player_c : u16,
    player_l : u16,

}

fn draw(mut sc: &Stdout , world: &World){
     sc.queue(MoveTo(world.player_c,world.player_l));
     sc.queue(Print("X"));
     sc.flush();

}

fn main() -> std::io::Result<()> {
        let mut sc = stdout();
        let (maxc , maxl) = size().unwrap();
        enable_raw_mode();


        let mut world = World{
            player_c : maxc / 2,
            player_l : maxl - 1,

        };
        loop{

            if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let key = read().unwrap();
            match key {
                Event::Key(event) => {
                        match event.code{
                            KeyCode::Char('q') => {break;}
                            _ => {}
                    }



                },
                _ => {},
            }
        } else {
            // Timeout expired and no `Event` is available
        }



             draw(&sc , &world);
        };
    disable_raw_mode();
    Ok(())
}
