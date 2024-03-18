use std::fmt::format;
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
    frame_time:f32,
    score:i32,
    obstacle:Obstacle, //障碍物
}
impl State{
    fn new()->Self{
        State{
            mode:GameMode::Menu,
            player:Player::new(0,0),
            frame_time:0.0,
            score:0,
            obstacle:Obstacle::new(SCREEN_WIDTH,0)
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

        //打印得分
        ctx.print(0,1,format!("Score :{}",self.score));

        self.obstacle.render(ctx,self.player.x);

        if self.player.x > self.obstacle.x{
            self.score +=1;
            //创建一个新的管子
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH,self.score);
        }


        self.player.render(ctx);

        if let Some(VirtualKeyCode::Space) = ctx.key  {
            self.player.fly_up()
        }
        //是否撞到管道
        if(self.player.y == SCREEN_HEIGHT || self.obstacle.hit_bostacle(&self.player)){
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
        self.x += 1;
        if(self.y<0){
            self.y = 0;
        }
    }

    fn fly_up(& mut self){
        if self.y > 0{
            self.velocity -= 0.8;
        }
    }
}


struct Obstacle{
    x:i32, //世界空间的横坐标
    gap_y:i32,
    size:i32
}

impl Obstacle{
    fn new(x:i32,score:i32)->Self{
        let mut random = RandomNumberGenerator::new();
        Obstacle{
            x,
            gap_y:random.range(10,40),
            size:i32::max(2,20-score)
        }
    }

    fn render(&mut self,ctx:&mut BTerm,player_x:i32){
        println!("obstacle render!! x:{}",self.x);
        //屏幕空间
        let screen_x = self.x - player_x;
        let half_size:i32 = self.size /2;
        for y in 0..self.gap_y - half_size{
            ctx.set(screen_x,y,RED,BLACK,to_cp437('|'))
        }
        for y in self.gap_y + half_size..SCREEN_HEIGHT{
            ctx.set(screen_x,y,RED,BLACK,to_cp437('|'))
        }
    }

    //碰撞
    fn hit_bostacle(&self,player:&Player)->bool{
        let half_size = self.size/2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && ( player_above_gap || player_below_gap )

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
