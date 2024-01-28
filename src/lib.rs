use median::{
    attr::{AttrBuilder, AttrType},
    builder::MaxWrappedBuilder,
    class::Class,
    max_sys::t_atom_long,
    num::Float64,
    object::MaxObj,
    post,
    object_post,
    object_error,
    atom::Atom,
    method,
    wrapper::{tramp, MaxObjWrapped, MaxObjWrapper},
};
use parking_lot::Mutex;
use client::ScreenClient;

mod point3d;
mod screen;
mod client;

//you need to wrap your external in this macro to get the system to register your object and
//automatically generate trampolines and what not.
median::external! {
    #[name="screen_pointer"]
    pub struct MaxExtern {
        top_left: Mutex<(f64, f64, f64)>,
        bottom_left: Mutex<(f64, f64, f64)>,
        bottom_right: Mutex<(f64, f64, f64)>,
        screen: Mutex<Option<screen::Screen>>,
        screen_client: Mutex<Option<ScreenClient>>,
    }

    //implement the max object wrapper
    impl MaxObjWrapped<MaxExtern> for MaxExtern {
        //create an instance of your object
        //setup inlets/outlets and clocks
        fn new(_builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
            Self {
                top_left: Mutex::new((0.0, 0.0, 0.0)),
                bottom_left: Mutex::new((0.0, 0.0, 0.0)),
                bottom_right: Mutex::new((0.0, 0.0, 0.0)),
                screen: Mutex::new(None),
                screen_client: Mutex::new(None),
            }
        }

        // Register any methods you need for your class
        fn class_setup(c: &mut Class<MaxObjWrapper<Self>>) {
            c.add_method(method::Method::SelFFF("top_left", Self::top_left_tramp, 0))
                .unwrap();
            c.add_method(method::Method::SelFFF("bottom_left", Self::bottom_left_tramp, 0))
                .unwrap();
            c.add_method(method::Method::SelFFF("bottom_right", Self::bottom_right_tramp, 0))
                .unwrap();
        }
    }

    //implement any methods you might want for your object that aren't part of the wrapper
    impl MaxExtern {
        // build screen
        #[bang]
        pub fn bang(&self) {
            let top_left = self.top_left.lock();
            let bottom_left = self.bottom_left.lock();
            let bottom_right = self.bottom_right.lock();
            let screen = match screen::Screen::new(
                point3d::Point3D::new(top_left.0, top_left.1, top_left.2),
                point3d::Point3D::new(bottom_left.0, bottom_left.1, bottom_left.2),
                point3d::Point3D::new(bottom_right.0, bottom_right.1, bottom_right.2),
            ) {
                Ok(screen) => screen,
                Err(e) => {
                    object_error!(self.max_obj(), "Error creating screen: {}", e);
                    return;
                }
            };
            object_post!(self.max_obj(), "Screen created");
            let mut screen_ref = self.screen.lock();
            *screen_ref = Some(screen);
            let mut screen_client_ref = self.screen_client.lock();
            if screen_client_ref.is_none() {
                match ScreenClient::new() {
                    Ok(screen_client) => {
                        *screen_client_ref = Some(screen_client);
                    }
                    Err(e) => {
                        object_error!(self.max_obj(), "Error creating screen client: {}", e);
                    }
                }
            }
        }

        #[list]
        pub fn list(&self, list: &[Atom]) {
            if list.len() == 6 {
                let mut screen_ref = self.screen.lock();
                if let Some(screen) = &mut *screen_ref {
                    let x0 = list[0].get_float();
                    let y0 = list[1].get_float();
                    let z0 = list[2].get_float();
                    let dx = list[3].get_float() - x0;
                    let dy = list[4].get_float() - y0;
                    let dz = list[5].get_float() - z0;
                    let (x, y) = screen.intercept(x0, y0, z0, dx, dy, dz);
                    let mut screen_client_ref = self.screen_client.lock();
                    if let Some(screen_client) = &mut *screen_client_ref {
                        screen_client.send((x, y));
                    }
                }
            }
        }

        #[tramp]
        pub fn top_left(&self, x: f64, y: f64, z: f64) {
            object_post!(self.max_obj(), "top_left: {}, {}, {}", x, y, z);
            let mut top_left = self.top_left.lock();
            *top_left = (x, y, z);
        }

        #[tramp]
        pub fn bottom_left(&self, x: f64, y: f64, z: f64) {
            object_post!(self.max_obj(), "bottom_left: {}, {}, {}", x, y, z);
            let mut bottom_left = self.bottom_left.lock();
            *bottom_left = (x, y, z);
        }

        #[tramp]
        pub fn bottom_right(&self, x: f64, y: f64, z: f64) {
            object_post!(self.max_obj(), "bottom_right: {}, {}, {}", x, y, z);
            let mut bottom_right = self.bottom_right.lock();
            *bottom_right = (x, y, z);
        }
    }
}
