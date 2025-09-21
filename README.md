# 🌌 N-Body Gravity Simulator (Rust)

This is my first attempt at building an **N-body simulator** in Rust.  
The idea is to simulate gravitational interactions between multiple bodies (like the Sun and Earth) using Newton’s law of universal gravitation.  

Right now, the project runs a simple **2-body system** (Sun + Earth) and prints out the positions and velocities over time.  

---

## 🚀 How to Run

```bash
git clone https://github.com/<your-username>/n-body-simulator.git
cd n-body-simulator

cargo run
````

The simulation will print the positions of all bodies at each timestep.

---

## ⚙️ How It Works

* Each body has:

  * Position `(x, y)`
  * Velocity `(vx, vy)`
  * Mass `m`

* At each timestep:

  1. Compute gravitational forces between all pairs of bodies.
  2. Calculate accelerations from the forces.
  3. Update velocities (`v += a * dt`).
  4. Update positions (`p += v * dt`).

This method is called the **Forward Euler integration scheme**.

---

## 📚 What I Learned

* How to model a physical system with structs and vectors in Rust.
* How to implement Newton’s law of gravitation in code.
* How simple numerical methods like Forward Euler can give quick results, but also have **accuracy and stability issues**.
* That orbital dynamics are very sensitive to the integration method: small numerical errors can accumulate over time.

---

## ⚠️ Known Limitations

Right now, the simulator uses **Forward Euler**, which is:

* Easy to implement ✔️
* Fast ✔️
* But **not stable** for long-term orbital dynamics ❌

This means that after many timesteps, the Earth might slowly spiral into the Sun or drift away into space, even if the initial conditions are correct.

A better approach for orbital mechanics would be Verlet / Velocity-Verlet, however i will keep it like this due to it being a learning project

## 📝 Example Output

```
Initial state of the universe:
Body 0 -> pos=(0.00e0, 0.00e0), vel=(0.00e0, 0.00e0), mass=1.99e30
Body 1 -> pos=(1.50e11, 0.00e0), vel=(0.00e0, 2.98e4), mass=5.97e24

Step    0 Earth -> pos=(1.50e11, 2.98e5), vel=(-5.93e-2, 2.98e4)
Step    1 Earth -> pos=(1.50e11, 5.96e5), vel=(-1.19e-1, 2.98e4)
Step    2 Earth -> pos=(1.50e11, 8.93e5), vel=(-1.78e-1, 2.98e4)
...
```