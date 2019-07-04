#[macro_export]
macro_rules! request_animation_frame {
    ($anim_body:block) => {
        use std::cell::RefCell;
        use std::rc::Rc;

        fn request_animation_frame(f: &Closure<dyn FnMut()>) {
            web_sys::window()
                .expect("no global `window` exists")
                .request_animation_frame(f.as_ref().unchecked_ref())
                .expect("should register `requestAnimationFrame` OK");
        }

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            $anim_body

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    };
}
