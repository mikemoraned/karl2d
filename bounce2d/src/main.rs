extern crate nalgebra as na;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate nphysics_testbed3d;

use na::{Isometry3, Point3, Vector3};            // For configuring and positioning bodies.
use ncollide3d::shape::{Cuboid, Ball, ShapeHandle};    // Shapes for colliders.
use nphysics3d::object::{BodyHandle, Material};  // Body handle and collider material.
use nphysics3d::volumetric::Volumetric;          // To retrieve the center of mass and inertia properties of a shape.
use nphysics3d::world::World;                    // The physics world to be initialized.
use nphysics_testbed3d::Testbed;                 // The testbed to display/run the simulation.

fn main() {
    let mut world = World::new();
    world.set_gravity(Vector3::y() * -9.81);
 
    // world.set_gravity(Vector3::y() * -9.81 / 4.0);

    const COLLIDER_MARGIN: f32 = 0.01;
    const RESTITUTION: f32 = 0.8;
    const FRICTION: f32 = 1.0;

    let ground_size = 50.0;
    let ground_shape =
        ShapeHandle::new(Cuboid::new(Vector3::repeat(ground_size - COLLIDER_MARGIN)));
    let ground_pos = Isometry3::new(Vector3::y() * -ground_size, na::zero());

    world.add_collider(
        COLLIDER_MARGIN,
        ground_shape,
        BodyHandle::ground(),
        ground_pos,
        // Material::default(),
        nphysics3d::object::Material::new(RESTITUTION, FRICTION),
    );

    // world.integration_parameters_mut().dt = (1.0/60.0) * 2.0;

    let num = 1; 
    let rad = 0.1;
    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;
    let height = 2.0;

    // let geom = ShapeHandle::new(Cuboid::new(Vector3::repeat(rad - COLLIDER_MARGIN)));
    let geom = ShapeHandle::new(Ball::new(rad - COLLIDER_MARGIN));
    let inertia = geom.inertia(1.0);
    // let inertia = geom.inertia(0.01);
    let center_of_mass = geom.center_of_mass();

    for i in 0usize..num {
        for j in 0usize..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + height;
                let z = k as f32 * shift - centerz;

                /*
                    * Create the rigid-body.
                    */
                let pos = Isometry3::new(Vector3::new(x, y, z), na::zero());
                let handle = world.add_rigid_body(pos, inertia, center_of_mass);

                /*
                * Create the collider and attach it to the body we just created.
                */
                world.add_collider(
                    COLLIDER_MARGIN,
                    geom.clone(),
                    handle,
                    Isometry3::identity(),
                    // Material::default(),
                    nphysics3d::object::Material::new(RESTITUTION, FRICTION),
                );
            }
        }
    }

    let mut testbed = Testbed::new(world);
    testbed.look_at(Point3::new(-4.0, 1.0, -4.0), Point3::new(0.0, 1.0, 0.0));
    testbed.run();
}