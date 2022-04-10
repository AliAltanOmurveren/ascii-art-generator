use std::env;
use image::{self};
use crossterm::terminal::size;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err(Error::new(ErrorKind::Other , "Not enough arguments!"))
    }else {

        // init
        let image_path = &args[1];
        let (term_width, term_height) = size()?;

        let invert = match args.get(2){
            Some(s) =>{
                if s == &String::from("invert"){
                    Ok(true)
                }else{
                    Err(Error::new(ErrorKind::Other , String::from(format!("Argument '{}' not found", args[2]))))
                }
            },
            None => Ok(false)
        };

        let invert = invert.unwrap();

        let img = image::open(image_path).unwrap();

        // convert every img to rgba
        let img = img.to_rgba8();

        let (img_w, img_h) = img.dimensions();
        
        // resize
        let res_fac = resize_factor(term_width, term_height, img_w, img_h);

        /*
         //Resize Debug
        println!("fac: {}, iw: {}, ih: {}, niw = {} nih = {}, tw: {}, th: {}",
                    res_fac, img_w, img_h, 
                    (img_w as f32 * res_fac) as u32,
                    (img_h as f32 * res_fac) as u32,
                    term_width,
                    term_height);
        */
        
        let img =image::imageops::resize(&img, 
                                                (img_w as f32 * res_fac * 2.0) as u32, 
                                                (img_h as f32 * res_fac) as u32, 
                                                image::imageops::FilterType::Nearest);

        img.save("check.png").unwrap();
                                                
        // ascii chars ascending pixel count
        let ascii = vec![' ', '.', ',', ':', '-', '~', '+', '=', '*', 'o', 'O', '8', '&', '%', '$', '#', '@'];

        // convert to grayscale and print
        for y in 0..img.dimensions().1{
            for x in 0..img.dimensions().0{

                let pixel = img.get_pixel(x, y);
                
                // convert pixels to grayscale
                let grayscale = pixel[0] as f32 * 0.21 + pixel[1] as f32 * 0.72 + pixel[2] as f32 * 0.07;
                let grayscale = grayscale as u8;

                //
                if pixel[3] > 0{
                    if invert{
                        // invert color
                        print!("{}", ascii[((255 - grayscale) / ascii.len() as u8) as usize]);
                    }else{
                        print!("{}", ascii[(grayscale / ascii.len() as u8) as usize]);
                    }
                }else{
                    print!(" ");
                }
            }

            print!("\n");
        }

        Ok(())
    }
}

fn resize_factor(t_w: u16, t_h: u16, i_w: u32, i_h: u32) -> f32{
    let res_fac = (t_w as f32 / (i_w as f32 * 2.0)) as f32;

    if i_h as f32 * res_fac <= t_h as f32{
        res_fac
    }else{
        t_h as f32 / i_h as f32
    }
}

    