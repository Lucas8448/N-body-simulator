#[derive(Debug, Clone, Copy)]
struct Body {
    pos: [f64; 2], // position (x, y)
    vel: [f64; 2], // velocity (vx, vy)
    mass: f64,     // mass
}

struct Universe {
    bodies: Vec<Body>,
    g: f64,   // gravitational constant
    dt: f64,  // timestep
}

impl Universe {
    fn new(bodies: Vec<Body>, g: f64, dt: f64) -> Self {
        Self { bodies, g, dt }
    }

    /// Advance the simulation one step
    fn step(&mut self) {
        // TODO: compute gravitational forces between all pairs of bodies
        // TODO: calculate accelerations from forces
        // TODO: update velocities using v += a * dt
        // TODO: update positions using p += v * dt
    }
}

fn main() {
    // Example: Sun and Earth (2-body system, simplified)
    let sun = Body {
        pos: [0.0, 0.0],
        vel: [0.0, 0.0],
        mass: 1.989e30, // kg
    };

    let earth = Body {
        pos: [1.496e11, 0.0],   // 1 AU away in meters
        vel: [0.0, 29_780.0],   // orbital velocity in m/s
        mass: 5.972e24,
    };

    let universe = Universe::new(vec![sun, earth], 6.67430e-11, 60.0);

    println!("Initial state of the universe:");
    for (i, body) in universe.bodies.iter().enumerate() {
        println!(
            "Body {} -> pos=({:.2e}, {:.2e}), vel=({:.2e}, {:.2e}), mass={:.2e}",
            i, body.pos[0], body.pos[1], body.vel[0], body.vel[1], body.mass
        );
    }

    // TODO: run simulation loop here
    // for _ in 0..1000 {
    //     universe.step();
    //     println!("{:?}", universe.bodies);
    // }
}
