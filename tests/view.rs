use eom::*;
use ndarray::*;

#[test]
fn arr() {
    let dt = 0.01;
    let eom = ode::Lorenz63::default();
    let mut teo = explicit::Euler::new(eom, dt);
    let mut x: Array1<f64> = arr1(&[1.0, 0.0, 0.0]);
    teo.iterate(&mut x);
}

#[test]
fn rcarr() {
    let dt = 0.01;
    let eom = ode::Lorenz63::default();
    let mut teo = explicit::Euler::new(eom, dt);
    let mut x: ArcArray<f64, Ix1> = rcarr1(&[1.0, 0.0, 0.0]);
    teo.iterate(&mut x);
}

#[test]
fn view_mut() {
    let dt = 0.01;
    let eom = ode::Lorenz63::default();
    let mut teo = explicit::Euler::new(eom, dt);
    let mut x: Array1<f64> = arr1(&[1.0, 0.0, 0.0]);
    let v = &mut x.view_mut();
    teo.iterate(v);
}
