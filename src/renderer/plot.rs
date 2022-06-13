use crate::executor::ExecOption;
/// Plotter crate use struct based implementation which makes it so hard to make plot rendering
/// very discrete.
///
/// The reason render processes doesn't look DRY is because creating generic chartcontext over
/// various chart types are not so easy and doesn't worth the hassle.
use crate::{error::GdeError, models::GdeResult};
use plotters::prelude::*;
use r4d::{Processor, RadStorage, StorageOutput, StorageResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::models::GRender;

pub struct PlotRenderer;

impl GRender for PlotRenderer {
    fn rad_setup(&self, processor: &mut Processor) -> GdeResult<()> {
        processor.set_storage(Box::new(PlotModel::default()));
        Ok(())
    }

    fn render(&self, processor: &mut Processor, option: &ExecOption) -> GdeResult<Option<PathBuf>> {
        // Extract storage method should always return Some
        // unless, it is a logic error
        let plot_model = if let Ok(Some(output)) = processor.extract_storage(true) {
            if let StorageOutput::Binary(bytes) = output {
                PlotModel::from_bytes(&bytes)?
            } else {
                return Err(GdeError::RendererError(
                    "Plotmodel cannot be constructed from StorageOutput::Text",
                ));
            }
        } else {
            return Err(GdeError::RendererError(
                "Plot needs porper macro setup to work. Failed to created plot map",
            ));
        };
        let out_file = if let Some(file) = &option.out_file {
            file.to_owned()
        } else {
            PathBuf::from("out.png")
        };

        let plot_type = plot_model.plot_type;

        match plot_type {
            PlotType::BarV => {
                self.bar_chart_vertical(out_file, plot_model)?;
            }
            PlotType::BarH => {
                self.bar_chart_horizontal(out_file, plot_model)?;
            }
            PlotType::Line => {
                self.line_chart(out_file, plot_model, false)?;
            }
            PlotType::Area => {
                self.line_chart(out_file, plot_model, true)?;
            }
            _ => (),
        }

        Ok(None)
    }
}

impl PlotRenderer {
    fn line_chart(&self, out_file: PathBuf, plot_model: PlotModel, fill: bool) -> GdeResult<()> {
        let root_area = BitMapBackend::new(&out_file, plot_model.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let (caption_font, caption_size) = plot_model.caption_style;
        let (desc_font, desc_size) = plot_model.desc_style;

        if plot_model.data.is_empty() {
            return Err(GdeError::RendererError("Plot data is empty"));
        }

        let max = plot_model
            .data
            .iter()
            .max_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap_or(&0f64);

        let row_line_end = plot_model.row_offset as f64 + max.ceil() as f64;
        let column_count = plot_model.column_offset as usize + plot_model.data.len();

        let mut ctx = ChartBuilder::on(&root_area)
            .x_label_area_size(plot_model.x_label_size)
            .y_label_area_size(plot_model.y_label_size)
            .margin(plot_model.margin)
            .caption(&plot_model.caption, (caption_font.as_str(), caption_size))
            .build_cartesian_2d(0..column_count, 0f64..row_line_end)
            .map_err(|_| GdeError::PlotError("Failed to create chart".to_string()))?;

        // Mesh configuration
        ctx.configure_mesh()
            // Remove x lines
            .disable_x_mesh()
            // Remove y lines
            .disable_y_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(plot_model.y_desc)
            .x_desc(plot_model.x_desc)
            .axis_desc_style((desc_font.as_str(), desc_size))
            .draw()
            .map_err(|_| GdeError::PlotError("Failed to configure mesh for chart".to_string()))?;

        // Area series
        if fill {
            ctx.draw_series(
                AreaSeries::new(
                    (0usize..).zip(plot_model.data.iter()).map(|(x, y)| (x, *y)),
                    0.0,
                    &RED.mix(0.2),
                )
                .border_style(&RED),
            )
            .map_err(|_| GdeError::PlotError("Failed to embed data into a chart".to_string()))?;
        }
        // Line series
        else {
            // TODO, Ok this works at least
            ctx.draw_series(LineSeries::new(
                (0..).zip(plot_model.data.iter()).map(|(x, y)| (x, *y)),
                &RED,
            ))
            .map_err(|_| GdeError::PlotError("Failed to embed data into a chart".to_string()))?;
        }

        Ok(())
    }

    fn bar_chart_vertical(&self, out_file: PathBuf, plot_model: PlotModel) -> GdeResult<()> {
        let root_area = BitMapBackend::new(&out_file, plot_model.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let (caption_font, caption_size) = plot_model.caption_style;
        let (desc_font, desc_size) = plot_model.desc_style;

        let max = plot_model
            .data
            .iter()
            .max_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap();

        let row_count = plot_model.row_offset as u32 + max.ceil() as u32;
        let column_count = plot_model.column_offset as u32 + plot_model.data.len() as u32;

        let mut chart = ChartBuilder::on(&root_area)
            .x_label_area_size(plot_model.x_label_size)
            .y_label_area_size(plot_model.y_label_size)
            .margin(plot_model.margin)
            .caption(&plot_model.caption, (caption_font.as_str(), caption_size))
            // Into_segmented makes number match column's center position
            .build_cartesian_2d((0..column_count).into_segmented(), 0u32..row_count)
            .map_err(|_| GdeError::PlotError("Failed to create chart".to_string()))?;

        // Mesh configuration
        chart
            .configure_mesh()
            // Remove x lines
            .disable_x_mesh()
            // Remove y lines
            .disable_y_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(plot_model.y_desc)
            .x_desc(plot_model.x_desc)
            .axis_desc_style((desc_font.as_str(), desc_size))
            .draw()
            .map_err(|_| GdeError::PlotError("Failed to configure mesh for chart".to_string()))?;

        let data = plot_model
            .data
            .iter()
            .map(|f| *f as u32)
            .collect::<Vec<u32>>();
        chart
            .draw_series((0..).zip(data.iter()).map(draw_bar_v))
            .unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root_area.present().expect("Unable to write result to file");

        Ok(())
    }

    fn bar_chart_horizontal(&self, out_file: PathBuf, plot_model: PlotModel) -> GdeResult<()> {
        let root_area = BitMapBackend::new(&out_file, plot_model.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let (caption_font, caption_size) = plot_model.caption_style;
        let (desc_font, desc_size) = plot_model.desc_style;

        let max = plot_model
            .data
            .iter()
            .max_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap();

        let row_count = plot_model.row_offset as u32 + max.ceil() as u32;
        let column_count = plot_model.column_offset as u32 + plot_model.data.len() as u32;

        let mut chart = ChartBuilder::on(&root_area)
            .x_label_area_size(plot_model.x_label_size)
            .y_label_area_size(plot_model.y_label_size)
            .margin(plot_model.margin)
            .caption(&plot_model.caption, (caption_font.as_str(), caption_size))
            // Into_segmented makes number match column's center position
            .build_cartesian_2d(0u32..row_count, (0..column_count).into_segmented())
            .map_err(|_| GdeError::PlotError("Failed to create chart".to_string()))?;

        // Mesh configuration
        chart
            .configure_mesh()
            // Remove x lines
            .disable_x_mesh()
            // Remove y lines
            .disable_y_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(plot_model.y_desc)
            .x_desc(plot_model.x_desc)
            .axis_desc_style((desc_font.as_str(), desc_size))
            .draw()
            .map_err(|_| GdeError::PlotError("Failed to configure mesh for chart".to_string()))?;

        let data = plot_model
            .data
            .iter()
            .map(|f| *f as u32)
            .collect::<Vec<u32>>();

        chart
            .draw_series((0..).zip(data.iter()).map(draw_bar_h))
            .unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root_area.present().expect("Unable to write result to file");

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotModel {
    plot_type: PlotType,
    caption: String,
    caption_style: (String, f64),
    x_desc: String,
    x_label_size: i32,
    y_desc: String,
    y_label_size: i32,
    data: Vec<f64>,
    background: Option<(u8, u8, u8)>,
    row_offset: u32,
    column_offset: u32,
    desc_style: (String, f64),
    img_size: (u32, u32),
    margin: i32,
}

impl Default for PlotModel {
    fn default() -> Self {
        Self {
            plot_type: PlotType::Line,
            caption: "Plot image".to_owned(),
            caption_style: ("Helvetica".to_owned(), 30.0),
            x_desc: "X label".to_owned(),
            x_label_size: 50,
            y_desc: "Y label".to_owned(),
            y_label_size: 50,
            data: vec![],
            background: None,
            row_offset: 0,
            column_offset: 0,
            desc_style: ("Helvetica".to_owned(), 20.0),
            img_size: (800, 500),
            margin: 30,
        }
    }
}

impl PlotModel {
    pub fn from_bytes(source: &[u8]) -> GdeResult<Self> {
        Ok(bincode::deserialize::<Self>(source).expect(""))
    }
}

impl RadStorage for PlotModel {
    fn update(&mut self, args: &[String]) -> StorageResult<()> {
        let target = args[0].trim();
        let value = args[1].trim();

        // Insert values
        match target {
            "plot_type" => {
                self.plot_type = value.into();
            }
            "caption" => {
                self.caption = value.to_owned();
            }
            "caption_style" => {
                let style = value.split_whitespace().collect::<Vec<&str>>();
                if style.len() != 2 {
                    eprintln!("Description style is not valid");
                    panic!();
                }
                self.caption_style = (
                    style[0].to_owned(),
                    style[1]
                        .trim()
                        .parse()
                        .expect("Failed to get desc style font size"),
                );
            }
            "x_desc" => {
                self.x_desc = value.to_owned();
            }
            "x_label_size" => {
                self.x_label_size = value.trim().parse().expect("Failed to parse x label size");
            }
            "y_desc" => {
                self.y_desc = value.to_owned();
            }
            "y_label_size" => {
                self.y_label_size = value.trim().parse().expect("Failed to parse y label size");
            }
            "data" => {
                self.data = value
                    .split(',')
                    .map(|datum| datum.trim().parse().expect("Failed to parse datum"))
                    .collect::<Vec<f64>>();
            }
            "background" => {
                let bg = value
                    .split_whitespace()
                    .map(|color_code| {
                        color_code
                            .trim()
                            .parse()
                            .expect("Failed to parse background color code")
                    })
                    .collect::<Vec<u8>>();

                if bg.len() != 3 {
                    eprintln!("Background color code is not valid");
                    panic!();
                }

                self.background.replace((bg[0], bg[1], bg[2]));
            }
            "row_offset" => {
                self.row_offset = value.trim().parse().expect("Failed to parse row offset");
            }
            "column_offset" => {
                self.column_offset = value.trim().parse().expect("Failed to parse column offset");
            }
            "desc_style" => {
                let style = value.split_whitespace().collect::<Vec<&str>>();
                if style.len() != 2 {
                    eprintln!("Description style is not valid");
                    panic!();
                }
                self.desc_style = (
                    style[0].to_owned(),
                    style[1]
                        .trim()
                        .parse()
                        .expect("Failed to get desc style font size"),
                );
            }
            "img_size" => {
                let size = value
                    .split_whitespace()
                    .map(|num| num.trim().parse().expect("Failed to parse img size"))
                    .collect::<Vec<u32>>();
                if size.len() != 2 {
                    eprintln!("Image size style is not valid");
                    panic!();
                }
                self.img_size = (size[0], size[1]);
            }
            "margin" => {
                self.margin = value.trim().parse().expect("Failed to parse margin value");
            }
            _ => (),
        }
        Ok(())
    }

    fn extract(&mut self, serialize: bool) -> StorageResult<Option<StorageOutput>> {
        if serialize {
            Ok(Some(StorageOutput::Binary(
                bincode::serialize(self).expect("Failed to print as string"),
            )))
        } else {
            Ok(Some(StorageOutput::Text(
                serde_json::to_string(self).expect("Failed to print as string"),
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PlotType {
    HistV, // Histogram vertical
    HistH, // Histogram horizontal
    BarV,  // Bar verrtical
    BarH,  // Bar horizontal
    Line,  // Line series
    Area,  // Aera series
    None,  // Fallback
}

impl Default for PlotType {
    fn default() -> Self {
        Self::Line
    }
}

impl From<&str> for PlotType {
    fn from(src: &str) -> Self {
        match src.trim().to_lowercase().as_str() {
            "histv" => Self::HistV,
            "histh" => Self::HistH,
            "barh" => Self::BarH,
            "barv" => Self::BarV,
            "line" => Self::Line,
            "area" => Self::Area,
            _ => Self::None,
        }
    }
}

// Dry methods
fn draw_bar_v(tup: (u32, &u32)) -> Rectangle<(plotters::prelude::SegmentValue<u32>, u32)> {
    let (x, y) = tup;
    let x0 = SegmentValue::Exact(x);
    let x1 = SegmentValue::Exact(x + 1);
    let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
    bar.set_margin(0, 0, 5, 5);
    bar
}

fn draw_bar_h(tup: (u32, &u32)) -> Rectangle<(u32, plotters::prelude::SegmentValue<u32>)> {
    let (y, x) = tup;
    let mut bar = Rectangle::new(
        [
            (0, SegmentValue::Exact(y)),
            (*x, SegmentValue::Exact(y + 1)),
        ],
        RED.filled(),
    );
    bar.set_margin(5, 5, 0, 0);
    bar
}
