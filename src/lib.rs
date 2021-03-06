mod utils;

extern crate wasm_bindgen;

// use serde::Deserialize;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
#[macro_use]
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[derive(Serialize, Deserialize)]
// struct Point {
//     x: f32,
//     y: f32,
//     i: i32,
// }
// }function closestPair(roundList) {
//   let res = [0, 1];
//   const [a, b] = res;
//   let closestDistance = Math.sqrt(
//     (roundList[a].x - roundList[b].x) ** 2 +
//       (roundList[a].y - roundList[b].y) ** 2
//   );
//   for (let i = 0; i < roundList.length; i++) {
//     for (let j = i + 1; j < roundList.length; j++) {
//       let distance = Math.sqrt(
//         (roundList[i].x - roundList[j].x) ** 2 +
//           (roundList[i].y - roundList[j].y) ** 2
//       );
//       if (distance < closestDistance) {
//         res = [i, j];
//         closestDistance = distance;
//       }
//     }
//   }
//   return res;
// }
fn distance(a: &RoundItem, b: &RoundItem) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    (x * x + y * y).sqrt()
}
// #[wasm_bindgen]
// pub fn calculate(points: &js_sys::Array) -> i32 {
//     let points: Vec<i32> = points.into_serde().unwrap();
//     let mut res = [0, 1];
//     let mut closestDistance = distance(&points[0], &points[1]);
//     for (ii, ip) in points.iter().enumerate() {
//         for (ji, jp) in points.iter().enumerate().skip(ii + 1) {
//             let distance = distance(ip, jp);
//             if distance < closestDistance {
//                 res = [ii, ji];
//                 closestDistance = distance;
//             }
//         }
//     }
//     res[0] as i32
// }

// class RoundItem {
//   constructor(index, x, y, speedX, speedY) {
//     this.index = index;
//     this.x = x;
//     this.y = y;
//     this.speedX = speedX;
//     this.speedY = speedY;
//     this.r = RADIUS;
//     this.color = STROKE_COLOR;
//   }
//   draw(fill) {
//     ctx.strokeStyle = this.color;
//     ctx.beginPath();
//     ctx.arc(this.x, this.y, this.r, 0, 2 * Math.PI, false);
//     ctx.closePath();
//     ctx.stroke();
//     if (fill) {
//       ctx.fill();
//     }
//   }
// }
#[wasm_bindgen]
pub struct RoundItem {
    x: f32,
    y: f32,
    r: f32,
    speed_x: f32,
    speed_y: f32,
}
#[wasm_bindgen]
pub struct RoundCanvas {
    width: i32,
    height: i32,
    r: f32,
    count: usize,
    roundList: Vec<RoundItem>,
    max_x: f32,
    max_y: f32,
}
fn in_area(x: f32, y: f32, r: f32, height: f32, width: f32) -> u8 {
    let res = 0 | (y - r >= 0.0 && y + r <= height) as u8;
    return (res << 1) | (x - r >= 0.0 && x + r <= width) as u8;
}
fn distance_point_tuple(a: &PointTuple, b: &PointTuple) -> f32 {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    (x * x + y * y).sqrt()
}

type PointTuple = (f32, f32, usize);
#[wasm_bindgen]
impl RoundCanvas {
    // function closestPair2Helper(px, py) {
    //   if (px.length <= 3) {
    //     return px.length === 3 ? closestFromThree(px) : px;
    //   }
    //   const mid = ~~(px.length / 2);
    //   const lx = px.slice(0, mid);
    //   const rx = px.slice(mid);
    //   const ly = [];
    //   const ry = [];
    //   const targetX = px[mid].x;
    //   for (let i = 0, length; i < length; i++) {
    //     if (py[i].x < targetX && ly.length < mid) {
    //       ly.push(py[i]);
    //     } else {
    //       ry.push(py[i]);
    //     }
    //   }
    //   let [l1, l2] = closestPair2Helper(lx, ly);
    //   let [r1, r2] = closestPair2Helper(rx, ry);
    //   let minPair;
    //   let min;
    //   // let min = Math.min(distance(l1, l2), distance(r1, r2));
    //   if (distance(l1, l2) < distance(r1, r2)) {
    //     min = distance(l1, l2);
    //     minPair = [l1, l2];
    //   } else {
    //     min = distance(r1, r2);
    //     minPair = [r1, r2];
    //   }
    //   let [s1, s2] = closestSplitPair(px, py, min);
    //   return s1
    //     ? distance(l1, l2) < distance(r1, r2)
    //       ? distance(l1, l2) < distance(s1, s2)
    //         ? [l1, l2]
    //         : [s1, s2]
    //       : distance(r1, r2) < distance(s1, s2)
    //       ? [r1, r2]
    //       : [s1, s2]
    //     : minPair;
    // }
    fn closest_pair_helper(
        &self,
        px: &[Rc<PointTuple>],
        py: &[Rc<PointTuple>],
    ) -> (Rc<PointTuple>, Rc<PointTuple>) {
        if px.len() <= 3 {
            if px.len() == 3 {
                return self.closest_from_three_point_tuple(px);
            } else {
                return (px[0].clone(), px[1].clone());
            };
        }
        let mid = px.len() / 2;
        let lx = &px[0..mid];
        let rx = &px[mid..];
        let mut ly = Vec::with_capacity(mid);
        let mut ry = Vec::with_capacity(px.len() - mid);
        // let mut ly= vec![];
        // let mut ry: Vec<PointTuple> = vec![];
        let pivot_x = px[mid].0;

        for item in py.iter() {
            if item.0 < pivot_x && ly.len() < mid {
                ly.push(item.clone());
            } else {
                ry.push(item.clone());
            }
        }
        let (l1, l2) = self.closest_pair_helper(&lx, &ly);
        let (r1, r2) = self.closest_pair_helper(&rx, &ry);
        let min;
        let minPair;
        let distancel1l2 = distance_point_tuple(&l1, &l2);
        let distancer1r2 = distance_point_tuple(&r1, &r2);
        if distancel1l2 < distancer1r2 {
            min = distancel1l2;
            minPair = (l1, r2);
        } else {
            min = distancer1r2;
            minPair = (r1, l2);
        }
        minPair
    }
    fn closest_from_three_point_tuple(
        &self,
        three_point_tuple: &[Rc<PointTuple>],
    ) -> (Rc<PointTuple>, Rc<PointTuple>) {
        let p0 = unsafe { three_point_tuple.get_unchecked(0) };
        let p1 = unsafe { three_point_tuple.get_unchecked(1) };
        let p2 = unsafe { three_point_tuple.get_unchecked(2) };
        let dis01 = distance_point_tuple(p0, p1);
        let dis12 = distance_point_tuple(p2, p1);
        let dis02 = distance_point_tuple(p0, p2);
        if dis01 < dis12 {
            if dis01 < dis02 {
                (p0.clone(), p1.clone())
            } else {
                (p0.clone(), p2.clone())
            }
        } else {
            if dis12 < dis02 {
                (p1.clone(), p2.clone())
            } else {
                (p0.clone(), p2.clone())
            }
        }
    }

    pub fn closest_pair_dc(&self) {
        let mut PX: Vec<Rc<PointTuple>> = self
            .roundList
            .iter()
            .enumerate()
            .map(|(index, item)| Rc::new((item.x, item.y, index)))
            .collect();
        let mut PY = PX.clone();
        PX.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        PY.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let (a, b) = self.closest_pair_helper(&PX, &PY);
    }
    pub fn closest_pair_brute(&self) -> Vec<usize> {
        let mut res = vec![0, 1];
        let round_list = &self.roundList;
        let mut closest_distance = distance(&round_list[0], &round_list[1]);
        for (ii, ip) in round_list.iter().enumerate() {
            for (ji, jp) in round_list.iter().enumerate().skip(ii + 1) {
                let distance = distance(ip, jp);
                if distance < closest_distance {
                    res = vec![ii, ji];
                    closest_distance = distance;
                }
            }
        }
        res
    }
    pub fn tick(&mut self) {
        let max_x = self.max_x;
        let max_y = self.max_y;
        let r = self.r;
        let height = self.height as f32;
        let width = self.width as f32;
        for item in self.roundList.iter_mut() {
            let next_x = item.x + item.speed_x;
            let next_y = item.y + item.speed_y;
            item.x = if next_x < r {
                r
            } else if next_x > max_x {
                max_x
            } else {
                next_x
            };
            item.y = if next_y < r {
                r
            } else if next_y > max_y {
                max_y
            } else {
                next_y
            };
            let flag = in_area(next_x, next_y, r, height, width);
            if (flag & 1) != 1 {
                item.speed_x *= -1.0;
            }
            if ((flag >> 1) & 1) != 1 {
                item.speed_y *= -1.0;
            }
        }
    }

    pub fn new(width: i32, height: i32, r: f32, count: usize) -> RoundCanvas {
        let mut roundList = vec![];
        let max_x = width as f32 - r;
        let max_y = height as f32 - r;
        for i in 0..count {
            let x = clamp(max_x, r, (js_sys::Math::random() * width as f64) as f32) as f32;
            let y = clamp(max_y, r, (js_sys::Math::random() * height as f64) as f32) as f32;
            let x_direction = if js_sys::Math::random() > 0.5 { 1 } else { -1 };
            let y_direction = if js_sys::Math::random() > 0.5 { 1 } else { -1 };
            let speed_x = (js_sys::Math::random() * 1.0 + 0.5) as f32 * x_direction as f32;
            let speed_y = (js_sys::Math::random() * 1.0 + 0.5) as f32 * y_direction as f32;
            roundList.push(RoundItem {
                r,
                speed_x,
                speed_y,
                x,
                y,
            });
        }
        RoundCanvas {
            width,
            height,
            r,
            count,
            roundList,
            max_x,
            max_y,
        }
    }

    pub fn rounds(&self) -> *const RoundItem {
        self.roundList.as_ptr()
    }
}

fn clamp<T: PartialEq + PartialOrd>(upbound: T, lowbound: T, value: T) -> T {
    if value < lowbound {
        lowbound
    } else if value > upbound {
        upbound
    } else {
        value
    }
}
