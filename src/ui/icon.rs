use druid::{
    kurbo::BezPath, Affine, BoxConstraints, Color, Env, Event, EventCtx, KeyOrValue, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget,
};

pub static WIFI: SvgIcon = SvgIcon {
  svg_path: "M15.384 6.115a.485.485 0 0 0-.047-.736A12.444 12.444 0 0 0 8 3C5.259 3 2.723 3.882.663 5.379a.485.485 0 0 0-.048.736.518.518 0 0 0 .668.05A11.448 11.448 0 0 1 8 4c2.507 0 4.827.802 6.716 2.164.205.148.49.13.668-.049z M13.229 8.271a.482.482 0 0 0-.063-.745A9.455 9.455 0 0 0 8 6c-1.905 0-3.68.56-5.166 1.526a.48.48 0 0 0-.063.745.525.525 0 0 0 .652.065A8.46 8.46 0 0 1 8 7a8.46 8.46 0 0 1 4.576 1.336c.206.132.48.108.653-.065zm-2.183 2.183c.226-.226.185-.605-.1-.75A6.473 6.473 0 0 0 8 9c-1.06 0-2.062.254-2.946.704-.285.145-.326.524-.1.75l.015.015c.16.16.407.19.611.09A5.478 5.478 0 0 1 8 10c.868 0 1.69.201 2.42.56.203.1.45.07.61-.091l.016-.015zM9.06 12.44c.196-.196.198-.52-.04-.66A1.99 1.99 0 0 0 8 11.5a1.99 1.99 0 0 0-1.02.28c-.238.14-.236.464-.04.66l.706.706a.5.5 0 0 0 .707 0l.707-.707z",
    svg_size: Size::new(16.0, 16.0),
};

pub struct WifiIcon {
    bez_path: BezPath,
    size: Size,
    scale: Affine,
}

impl WifiIcon {
    pub fn scale(to_size: impl Into<Size>) -> WifiIcon {
        let to_size = to_size.into();
        let bez_path = BezPath::from_svg(WIFI.svg_path).expect("Failed to parse SVG");
        let scale = Affine::scale_non_uniform(
            to_size.width / WIFI.svg_size.width,
            to_size.height / WIFI.svg_size.height,
        );
        WifiIcon {
            bez_path,
            size: to_size,
            scale,
        }
    }
}

impl Widget<u8> for WifiIcon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut u8, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &u8, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &u8, _data: &u8, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &u8,
        _env: &Env,
    ) -> Size {
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &u8, _env: &Env) {
        let color = Color::grey(*data as f64 / (100 as f64));
        ctx.with_save(|ctx| {
            ctx.transform(self.scale);
            ctx.fill(self.bez_path.clone(), &color);
        });
    }
}

#[derive(Clone)]
pub struct SvgIcon {
    svg_path: &'static str,
    svg_size: Size,
}

impl SvgIcon {
    pub fn scale(&self, to_size: impl Into<Size>) -> Icon {
        let to_size = to_size.into();
        let bez_path = BezPath::from_svg(self.svg_path).expect("Failed to parse SVG");
        let scale = Affine::scale_non_uniform(
            to_size.width / self.svg_size.width,
            to_size.height / self.svg_size.height,
        );
        Icon::new(
            bez_path,
            to_size,
            scale,
            KeyOrValue::from(Color::rgb8(0, 128, 0)),
        )
    }
}

#[derive(Clone)]
pub struct Icon {
    bez_path: BezPath,
    size: Size,
    scale: Affine,
    color: KeyOrValue<Color>,
}

impl Icon {
    pub fn new(bez_path: BezPath, size: Size, scale: Affine, color: KeyOrValue<Color>) -> Self {
        Icon {
            bez_path,
            size,
            scale,
            color,
        }
    }

    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }
}

impl<T> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let color = self.color.resolve(env);
        ctx.with_save(|ctx| {
            ctx.transform(self.scale);
            ctx.fill(&self.bez_path, &color);
        });
    }
}
