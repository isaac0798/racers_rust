use nannou::prelude::*;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64
}

struct Player {
    name: String,
    speed: f64,
    position: Position
}

struct Ball {
    position: Position
}

struct Pitch {
    height: f32,
    width: f32
}

fn main() {
    let tyreek_hill = Player {
        name: String::from("tyreek"),
        speed: 99.0,
        position: Position {
            x: 0.0,
            y: 0.0,
        }
    };
    let me = Player {
        name: String::from("isaac"),
        speed: 88.0,
        position: Position {
            x: 100.0,
            y: 0.0,
        }
    };

    let ball = Ball {
        position: Position {
            x: 100.0,
            y: 20.0
        }
    };

    let winner = race(tyreek_hill, me, ball);

    println!("{:?} has won", winner.name);

    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App,  _model: &mut Model, _update: Update) {

}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let pitch = Pitch {
        height: 600.0,
        width: 400.0
    };
    let draw = _app.draw();

    draw.background().color(BLACK);

    // draw pitch
    let top_left_corner    = pt2(-(pitch.height / 2.0), pitch.width / 2.0);
    let bottom_left_corner = pt2(-(pitch.height / 2.0), -(pitch.width / 2.0));

    let bottom_right_corner   = pt2(pitch.height / 2.0, -(pitch.width / 2.0));
    let top_right_corner = pt2(pitch.height / 2.0, pitch.width / 2.0);

    draw.line().start(bottom_left_corner).end(top_left_corner).color(WHITE);
    draw.line().start(bottom_right_corner).end(top_right_corner).color(WHITE);
    draw.line().start(bottom_right_corner).end(bottom_left_corner).color(WHITE);
    draw.line().start(top_left_corner).end(top_right_corner).color(WHITE);

    // draw players
    draw.ellipse().color(WHITE).w(10.0).h(10.0);


    draw.to_frame(_app, &frame).unwrap();
}

fn race(player1: Player, player2: Player, ball: Ball) -> Player {
    let player1_time = calc_time_to_run(&player1, &ball);
    let player2_time = calc_time_to_run(&player2, &ball);

    if player1_time > player2_time {
        return player2;
    }

    return player1;
}

fn calc_time_to_run(player: &Player, ball: &Ball) -> f64 {
    let x_distance = (player.position.x - ball.position.x).powf(2.0);
    let y_distance = (player.position.y - ball.position.y).powf(2.0);

    let distance = (x_distance + y_distance).sqrt();
    
    return distance / player.speed
}
