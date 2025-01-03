use web_sys::HtmlCanvasElement;



pub trait StarRender {
    fn update_and_render(&mut self, delta_time: f32);
    fn resize(&mut self, canvas: HtmlCanvasElement);
    fn add_stars(&mut self, count: u32);
    fn remove_stars(&mut self, count: u32);
}
