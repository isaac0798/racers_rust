use nannou::prelude::*;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32
}

struct Player {
    name: String,
    speed: f32,
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
        height: (120.0 * 3.0),
        width: (80.0 * 3.0)
    };
    let draw = _app.draw();

    draw.background().color(BLACK);
    draw.ellipse().color(WHITE).w(10.0).h(10.0);
    // draw pitch
    let bottom_left_corner = pt2(0.0, 0.0);
    let bottom_right_corner   = pt2(pitch.height, 0.0);
    let top_left_corner    = pt2(0.0, pitch.width);
    let top_right_corner = pt2(pitch.height, pitch.width);

    draw.line().start(bottom_left_corner).end(top_left_corner).color(WHITE);
    draw.line().start(bottom_right_corner).end(top_right_corner).color(WHITE);
    draw.line().start(bottom_right_corner).end(bottom_left_corner).color(WHITE);
    draw.line().start(top_left_corner).end(top_right_corner).color(WHITE);

    // draw players
    // currently speed is how long it takes to get from A -> B and distance doesnt matter
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
        speed: 99.0,
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

    draw.ellipse().color(WHITE).w(5.0).h(5.0).x_y(ball.position.x * 3.0, ball.position.y * 3.0);


    let x = map_range(run(&me, _app.time), 0.0, calc_time_to_run(&me, &ball), me.position.x * 3.0, ball.position.x * 3.0);
    let y = map_range(run(&me, _app.time), 0.0, calc_time_to_run(&me, &ball), me.position.y * 3.0, ball.position.y * 3.0);
    draw.ellipse().color(WHITE).w(10.0).h(10.0).x_y(x, y);

    let x_tyreek = map_range(run(&tyreek_hill, _app.time), 0.0, calc_time_to_run(&tyreek_hill, &ball), tyreek_hill.position.x * 3.0, ball.position.x * 3.0);
    let y_tyreek = map_range(run(&tyreek_hill, _app.time), 0.0, calc_time_to_run(&tyreek_hill, &ball), tyreek_hill.position.y * 3.0, ball.position.y * 3.0);
    draw.ellipse().color(WHITE).w(10.0).h(10.0).x_y(x_tyreek, y_tyreek);

    draw.to_frame(_app, &frame).unwrap();
}

fn run(player: &Player, time: f32) -> f32 {
    let distance = player.speed * 0.01 * time;

    if distance > 1.0 {
        return 1.0;
    } else if distance < 0.0 {
        return 0.0;
    }

    return distance;

}

fn calc_time_to_run(player: &Player, ball: &Ball) -> f32 {
    let x_distance = (player.position.x - ball.position.x).powf(2.0);
    let y_distance = (player.position.y - ball.position.y).powf(2.0);

    let distance = (x_distance + y_distance).sqrt();
    
    return distance / player.speed
}
