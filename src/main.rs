use bracket_lib::prelude::*;

enum GameMode{
    Menu,
    Playing,
    End
}

const SCREEN_WIDTH: i32 = 80;

const SCREEN_HEIGHT:i32 = 50;

// 控制猪飞行的速度 因为 tick太快了
const FRAME_DURATION:i32 = 75;
struct  State{
    mode:GameMode,
    player:Player,
    frame_time:f32
}
impl State{
    fn new()->Self{
        State{
            mode:GameMode::Menu,
            player:Player::new(0,0),
            frame_time:0.0
        }
    }
    fn main_menu(&mut self, ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5,"Welcome To Flappy Pig");
        ctx.print_centered(8,"(P) Play Game");
        ctx.print_centered(9,"(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P=>self.restart(),
                VirtualKeyCode::Q=>ctx.quitting = true,
                _=>{}
            }
        }
    }
    fn dead(&mut self, ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5,"Game Over ");
        ctx.print_centered(6,"(R) Restart");
        ctx.print_centered(7,"(Q) Quit");
        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::R=>self.restart(),
                VirtualKeyCode::Q=>ctx.quitting = true,
                _=>{}
            }
        }
    }
    fn playing(&mut self,ctx:& mut BTerm){

        // self.mode = GameMode::End;
        ctx.cls();
        self.frame_time+=ctx.frame_time_ms;
        if(self.frame_time > FRAME_DURATION as f32){
            self.frame_time = 0.0;
            self.player.change_gravity();
        }

        ctx.print(0,0,"Press Space to Flap");

        self.player.render(ctx);

        if let Some(VirtualKeyCode::Space) = ctx.key  {
            self.player.fly_up()
        }

        if(self.player.y == SCREEN_HEIGHT){
            self.mode = GameMode::End
        }

    }
    fn restart(& mut self){
        self.mode= GameMode::Playing
    }
}

// 飞翔的猪
struct Player{
    x:i32,
    y:i32,
    velocity:f32
}


impl Player{
    fn new(x:i32,y:i32)->Self{
        Player{
            x,
            y,
            velocity:0.0
        }
    }

    fn render(&mut self,ctx:&mut BTerm){
        ctx.set(self.x,self.y,YELLOW,BLACK,to_cp437('@'));
    }

    fn change_gravity(&mut self){
        if(self.velocity < SCREEN_HEIGHT as f32){
          self.velocity += 0.4;
        }
        self.y = self.velocity as i32;
        // self.x += 1;
        if(self.y<0){
            self.y = 0;
        }
    }

    fn fly_up(& mut self){
        println!("{}",self.velocity);
        if(self.y > 0){
            self.velocity -= 0.8;
        }
    }
}

impl GameState for State{
    fn tick(& mut self,ctx:&mut BTerm){
       match self.mode{
           GameMode::Menu=>self.main_menu(ctx),
           GameMode::End=>self.dead(ctx),
           GameMode::Playing=>self.playing(ctx)
       }
    }
}


fn main()->BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flayppy Pig")
        .build()?;
    main_loop(context,State::new())
}
