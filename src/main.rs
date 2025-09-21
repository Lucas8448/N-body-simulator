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
        let n = self.bodies.len();
        let mut accelerations = vec![[0.0, 0.0]; n];

        // Calculate pairwise forces
        for i in 0..n {
            for j in (i + 1)..n {
                let dx = self.bodies[j].pos[0] - self.bodies[i].pos[0];
                let dy = self.bodies[j].pos[1] - self.bodies[i].pos[1];
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();

                // Avoid division by zero
                if dist == 0.0 {
                    continue;
                }

                // gravitation
                let force = self.g * self.bodies[i].mass * self.bodies[j].mass / dist_sq;

                // Force direction
                let fx = force * dx / dist;
                let fy = force * dy / dist;

                // Acceleration = F/m
                accelerations[i][0] += fx / self.bodies[i].mass;
                accelerations[i][1] += fy / self.bodies[i].mass;
                accelerations[j][0] -= fx / self.bodies[j].mass;
                accelerations[j][1] -= fy / self.bodies[j].mass;
            }
        }

        // Update velocities and positions
        for i in 0..n {
            self.bodies[i].vel[0] += accelerations[i][0] * self.dt;
            self.bodies[i].vel[1] += accelerations[i][1] * self.dt;
            self.bodies[i].pos[0] += self.bodies[i].vel[0] * self.dt;
            self.bodies[i].pos[1] += self.bodies[i].vel[1] * self.dt;
        }
    }
}

fn main() {
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

    let mut universe = Universe::new(vec![sun, earth], 6.67430e-11, 10.0);

    println!("Initial state of the universe:");
    for (i, body) in universe.bodies.iter().enumerate() {
        println!(
            "Body {} -> pos=({:.2e}, {:.2e}), vel=({:.2e}, {:.2e}), mass={:.2e}",
            i, body.pos[0], body.pos[1], body.vel[0], body.vel[1], body.mass
        );
    }

    for step in 0..1000 {
        universe.step();
        let earth = &universe.bodies[1];
        println!(
            "Step {:4} Earth -> pos=({:.2e}, {:.2e}), vel=({:.2e}, {:.2e})",
            step, earth.pos[0], earth.pos[1], earth.vel[0], earth.vel[1]
        );
    }
}