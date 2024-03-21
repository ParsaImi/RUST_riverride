use std::io::{stdout, Write, Stdout};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    QueueableCommand,
    event,
    terminal::size,
    cursor::MoveTo,
};


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
        


        let mut world = World{
            player_c : maxc / 2,
            player_l : maxl - 1,

        };
        loop{
             draw(&sc , &world);
        };
    
    Ok(())
}
