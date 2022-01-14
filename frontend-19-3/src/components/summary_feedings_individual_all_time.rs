use chrono::{prelude::*, Duration};

use std::collections::BTreeMap;
use std::ops::Sub;
use std::rc::Rc;

use yew::prelude::*;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person::Person as ost_Person;

use itertools::Itertools;

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Properties)]
pub struct PropsSummaryIndividualAllTime {
    pub person: Rc<Box<dyn ost_Person>>,
    pub id: u32,
}

impl PartialEq for PropsSummaryIndividualAllTime {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub enum MsgSummaryFeedingsIndividualAllTime {
    MsgDataReceived(BTreeMap<Date<Utc>, DailyFeedingsAccumulator>),
}

pub struct SummaryFeedingsIndividualAllTime {
    props: PropsSummaryIndividualAllTime,
    canvas_all_time: NodeRef,
    person_name: String,
    data: BTreeMap<Date<Utc>, DailyFeedingsAccumulator>,
    is_loading: bool,
}

impl Component for SummaryFeedingsIndividualAllTime {
    type Message = MsgSummaryFeedingsIndividualAllTime;
    type Properties = PropsSummaryIndividualAllTime;

    fn create(ctx: &Context<Self>) -> Self {
        let p = ctx.props().person.clone();

        ctx.link().send_future(async move {
            let remote = AsyncRemoteMonolith {};
            let person_events = remote.feedings_by(&p).await;

            let mut accumulated_feedings = BTreeMap::<Date<Utc>, DailyFeedingsAccumulator>::new();

            for (key, group) in &person_events.iter().group_by(|key| key.time_stamp().date()) {
                let mut accum = DailyFeedingsAccumulator::default();
                group.for_each(|feed| {
                    accum.solids += feed.solids() as u64;
                    accum.breast_milk += feed.breast_milk() as u64;
                    accum.formula += feed.formula() as u64;
                });
                accum.total = accum.breast_milk + accum.formula + accum.solids;
                accumulated_feedings.insert(key, accum);
            }
            MsgSummaryFeedingsIndividualAllTime::MsgDataReceived(accumulated_feedings)
        });

        Self {
            props: ctx.props().clone(),
            canvas_all_time: NodeRef::default(),
            person_name: ctx.props().person.name().to_string(),
            is_loading: true,
            data: BTreeMap::<Date<Utc>, DailyFeedingsAccumulator>::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSummaryFeedingsIndividualAllTime::MsgDataReceived(d) => {
                self.is_loading = false;
                self.data = d;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let div_id = format!("canvas-container-{}", self.props.id);
        let canvas_all_time_id = format!("canvas-all-time-{}", self.props.id);

        let mut script = html! {};
        if !self.is_loading {
            script = html! {
                <script>
                    {
                        format!(
                        "
                        var cont = document.getElementById('canvas-container-{}');
                        var canv_all_time = document.getElementById('canvas-all-time-{}');
                        if (canv_all_time !== null ) {{
                            canv_all_time.width = cont.offsetWidth;
                            canv_all_time.height = cont.offsetWidth / 1.777;
                        }}
                        "
                        , self.props.id, self.props.id)
                    }
                </script>
            };
        }

        html! {
        <div id={div_id} class="block">
            <p>
                { format!("{}: all time feedings", self.person_name).clone() }
            </p>
            <canvas id={canvas_all_time_id} ref={self.canvas_all_time.clone()}>
            </canvas>
            { script }
        </div>
        }
    }

    // https://plotters-rs.github.io/book/intro/introduction.html
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            return;
        }

        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas_all_time = self.canvas_all_time.cast::<HtmlCanvasElement>().unwrap();
        let _res = self.draw_all_time(canvas_all_time, &self.data);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DailyFeedingsAccumulator {
    pub breast_milk: u64,
    pub formula: u64,
    pub solids: u64,
    pub total: u64,
}

// https://plotters-rs.github.io/book/basic/chart_components.html
impl SummaryFeedingsIndividualAllTime {
    pub fn draw_all_time(
        &self,
        canvas: HtmlCanvasElement,
        events: &BTreeMap<Date<Utc>, DailyFeedingsAccumulator>,
    ) -> DrawResult<()> {
        let area = CanvasBackend::with_canvas_object(canvas)
            .unwrap()
            .into_drawing_area();
        area.fill(&WHITE.mix(0.75))?;

        let max_total: i32 = match events.iter().map(|(_k, v)| v.total).max() {
            Some(t) => {
                if t < 100 {
                    100
                } else {
                    (1.025 * t as f64) as i32
                }
            }
            None => 1000,
        };

        let total_daily_feeds = events.len() as i32;

        let (starting_point, today) = (
            Utc::today().sub(Duration::days(total_daily_feeds as i64)),
            Utc::today(),
        );

        // Checking for enough data to render
        if total_daily_feeds == 0 {
            return Ok(());
        }

        let mut ctx = ChartBuilder::on(&area)
            .set_label_area_size(LabelAreaPosition::Right, 42)
            .set_label_area_size(LabelAreaPosition::Bottom, 22)
            .build_cartesian_2d(starting_point..today, 0..max_total)
            .unwrap();

        let labels: Vec<String> = vec![
            "Breast".to_string(),
            "Formula".to_string(),
            "Solids".to_string(),
            "Total".to_string(),
        ];

        ctx.configure_mesh()
            .x_labels(3)
            .x_label_formatter(&|o| o.format("%b/%d").to_string())
            .draw()
            .unwrap();

        ctx.draw_series(LineSeries::new(
            (0..total_daily_feeds).rev().map(|x: i32| {
                let date = Utc::today().sub(Duration::days(x as i64));
                let mut value: u64 = 0;
                if events.contains_key(&date) {
                    value = events.get(&date).unwrap().breast_milk;
                }
                (date, value as i32)
            }),
            &GREEN,
        ))
        .unwrap()
        .label(&labels[0])
        .legend(move |(x, y)| Rectangle::new([(x, y - 6), (x + 12, y + 6)], GREEN.filled()));

        ctx.draw_series(LineSeries::new(
            (0..total_daily_feeds).rev().map(|x: i32| {
                let date = Utc::today().sub(Duration::days(x as i64));
                let mut value: u64 = 0;
                if events.contains_key(&date) {
                    value = events.get(&date).unwrap().formula;
                }
                (date, value as i32)
            }),
            &RED,
        ))
        .unwrap()
        .label(&labels[1])
        .legend(move |(x, y)| Rectangle::new([(x, y - 6), (x + 12, y + 6)], RED.filled()));

        ctx.draw_series(LineSeries::new(
            (0..total_daily_feeds).rev().map(|x: i32| {
                let date = Utc::today().sub(Duration::days(x as i64));
                let mut value: u64 = 0;
                if events.contains_key(&date) {
                    value = events.get(&date).unwrap().solids;
                }
                (date, value as i32)
            }),
            &BLUE,
        ))
        .unwrap()
        .label(&labels[2])
        .legend(move |(x, y)| Rectangle::new([(x, y - 6), (x + 12, y + 6)], BLUE.filled()));

        ctx.draw_series(AreaSeries::new(
            (0..total_daily_feeds).rev().map(|x: i32| {
                let date = Utc::today().sub(Duration::days(x as i64));
                let mut value: u64 = 0;
                if events.contains_key(&date) {
                    value = events.get(&date).unwrap().total;
                }
                (date, value as i32)
            }),
            0,
            &RED.mix(0.33),
        ))
        .unwrap()
        .label(&labels[3])
        .legend(move |(x, y)| {
            Rectangle::new([(x, y - 6), (x + 12, y + 6)], RED.mix(0.33).filled())
        });

        ctx.configure_series_labels()
            .position(SeriesLabelPosition::LowerLeft)
            .background_style(&WHITE.mix(0.75))
            .border_style(&BLACK.mix(0.5))
            .legend_area_size(22)
            .draw()
            .unwrap();

        Ok(())
    }
}
