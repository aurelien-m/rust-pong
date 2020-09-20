use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::pong::{Ball, Side, Paddle, ARENA_HEIGHT};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        // checking for each ball:
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // - top or bottom of screen contact
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0) 
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0) {
                ball.velocity[1] = -ball.velocity[1];
            }

            // - contact with each paddles
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                // check if center of ball is in a radius augmented rectangle around paddle
                if point_in_rect(
                    ball_x, 
                    ball_y, 
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

// check is a (x, y) point is in a (left, bottom, right, top) rectangle
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}