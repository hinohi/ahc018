use crate::{
    gradient,
    math::vectors::{Vector, Vector2},
    permutationtable::NoiseHasher,
};

#[inline(always)]
pub fn perlin_surflet_2d<NH>(point: [f64; 2], hasher: &NH) -> f64
where
    NH: NoiseHasher + ?Sized,
{
    const SCALE_FACTOR: f64 = 3.160_493_827_160_493_7;

    fn surflet(index: usize, distance: Vector2<f64>) -> f64 {
        let attn: f64 = 1.0 - distance.magnitude_squared();

        if attn > 0.0 {
            let gradient = Vector2::from(gradient::grad2(index));
            attn.powi(4) * distance.dot(gradient)
        } else {
            0.0
        }
    }

    let point = Vector2::from(point);

    let floored = point.floor();
    let corner = floored.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_surflet(
        ($x:expr, $y:expr) => {
            {
                let offset = Vector2::new($x, $y);
                let index = hasher.hash(&(corner + offset).into_array());
                surflet(index, distance - offset.numcast().unwrap())
            }
        }
    );

    let f00 = call_surflet!(0, 0);
    let f10 = call_surflet!(1, 0);
    let f01 = call_surflet!(0, 1);
    let f11 = call_surflet!(1, 1);

    // Multiply by arbitrary value to scale to -1..1
    ((f00 + f10 + f01 + f11) * SCALE_FACTOR).clamp(-1.0, 1.0)
}
