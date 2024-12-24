use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use ratatui::crossterm;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use std::time::Duration;
use std::{fs, io, process, thread};
use std::io::{Stdout, Write};
use std::process::Command;


use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
fn extract_frames(video_path: &str, output_dir: &str) {
   
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    
    let output = Command::new("ffmpeg")
        .args([
            "-i",
            video_path,          
            "-vf",
            "fps=30",            
            format!("{}/frame%04d.png", output_dir).as_str(), 
        ])
        .output()
        .expect("Failed to execute FFmpeg command");

 
    if output.status.success() {
        println!("Frames extracted successfully!");
    } else {
        println!("Failed to extract frames: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn process_image(image_path: &str,emoji_lenth:u32) -> String {
   
    let img = image::open(image_path).expect("Failed to open image");

    let mut emojis = String::new();

   
    let rgb_img = img.to_rgb8();

   
    let (width, height) = rgb_img.dimensions();
    
    let block_size = width/(emoji_lenth/2);//方块宽度
    
    
    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let mut black_count = 0;
            let mut total_count = 0;

            
            for by in 0..block_size {
                for bx in 0..block_size {
                    let pixel_x = x + bx;
                    let pixel_y = y + by;

                    if pixel_x < width && pixel_y < height {
                        let pixel = rgb_img.get_pixel(pixel_x, pixel_y);
                        let r = pixel[0];
                        let g = pixel[1];
                        let b = pixel[2];

                        
                        if r == 0 && g == 0 && b == 0 {
                            black_count += 1;
                        }
                        total_count += 1;
                    }
                }
            }

            
            let black_ratio = black_count as f32 / total_count as f32;

            
            let emoji = match black_ratio {
                r if r >= 0.9 => '🌑', // 新月
                r if r >= 0.7 => '🌒', // 娥眉月
                r if r >= 0.5 => '🌓', // 上弦月
                r if r >= 0.3 => '🌔', // 盈凸月
                r if r >= 0.1 => '🌕', // 满月
                _ => '🌖',             // 亏凸月
            };

            
            emojis.push(emoji);
        }
        emojis.push('\n'); 
    }

    emojis

}

fn main() {
    
    let video_path = "badapple.mp4";

    
    let output_dir = "frames";

    // 提取视频帧!
    extract_frames(video_path, output_dir);

    
    let frame_files = fs::read_dir(output_dir).expect("Failed to read frames directory");



    let mut terminal = ratatui::init();




    for frame_file in frame_files {
        let frame_path = frame_file.expect("Failed to read frame file").path();
        let frame_path_str = frame_path.to_str().expect("Invalid frame path");
        let emoji_lenth= terminal.size().unwrap().width;
        
        let emoji = process_image(frame_path_str,emoji_lenth as u32);
        //println!("{}", emoji);
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.area());

            let paragraph = Paragraph::new(Text::raw(emoji))
                .block(Block::default().borders(ratatui::widgets::Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(ratatui::prelude::Alignment::Center);

            f.render_widget(paragraph, chunks[0]);
        }).unwrap();
       
    }
    ratatui::restore();
    
}