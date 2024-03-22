use std::io::{stdout, Write, Stdout, Result};
use std::{time::Duration, io};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    QueueableCommand,
    event,
    terminal::size,
    cursor::{MoveTo,Hide,Show},

};

use crossterm::event::{poll, read, Event};
use crossterm::event::KeyCode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
struct World{
    player_c : u16,
    player_l : u16,
    maxc : u16,
    maxl : u16,
    map : Vec<(u16,u16)>,
    died : bool,
}

fn physics(mut world :World) -> Result<World>{
    if world.player_c <= world.map[world.player_l as usize].0 ||
        world.player_c >= world.map[world.player_l as usize].1{
         world.died = true;
        }
    Ok(world)
}




fn draw(mut sc: &Stdout , world: &World) -> std::io::Result<()> {
     sc.queue(Clear(ClearType::All))?;
     // draw map
     for l in 0..world.map.len(){
        sc.queue(MoveTo(0,l as u16))?; 
        sc.queue(Print(".".repeat(world.map[l].0 as usize)))?;
        sc.queue(MoveTo(world.map[l].1,l as u16))?;                                                   sc.queue(Print(".".repeat((world.maxc - world.map[l].1) as usize)))?;
     }
     // draw player
     sc.queue(MoveTo(world.player_c,world.player_l))?;
     sc.queue(Print("X"))?;
     sc.flush()?;
     Ok(())

}

fn main() -> std::io::Result<()> {
        let mut sc = stdout();
        let (maxc , maxl) = size().unwrap();
        sc.execute(Hide)?;
        enable_raw_mode();


        let mut world = World{
            player_c : maxc / 2,
            player_l : maxl - 1,
            maxc : maxc,
            maxl : maxl,
            map : vec![ ((maxc/2)-5 , (maxc/2)+5); maxl as usize ],
            died : false,

        };
        while !world.died{

            if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let key = read().unwrap();
            match key {
                Event::Key(event) => {
                        match event.code{
                            KeyCode::Char('q') => {break;}
                            KeyCode::Char('w') => {
                                if world.player_l > 1 {world.player_l -= 1}
                            }
                            KeyCode::Char('s') => {                                           
                                if world.player_l < maxl-1 {world.player_l += 1}                   
                            }
                            KeyCode::Char('a') => {                                           
                                if world.player_c > 1 {world.player_c -= 1}
                            }
                            KeyCode::Char('d') => {
                                if world.player_c < maxc-1 {world.player_c += 1}


                            }
                            _ => {}
                    }



                },
                _ => {},
            }
        } else {
            // Timeout expired and no `Event` is available
        }


            world =  physics(world).unwrap();
             draw(&sc , &world);
        };
    sc.execute(Show)?;
    disable_raw_mode()?;
    sc.execute(Clear(ClearType::All))?;
    sc.execute(Print("Thanks Parsa You Done me Urself :>"))?;
    Ok(())
}
