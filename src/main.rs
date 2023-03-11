use clap::Parser;
use av_metrics_decoders::{VapoursynthDecoder, Decoder, Pixel};
use std::path::{Path, PathBuf};
use yuvxyb::{Hsl, LinearRgb, Rgb, Yuv, YuvConfig, TransferCharacteristic, ColorPrimaries, MatrixCoefficients};
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// input file
   #[arg(short, long)]
   source: PathBuf,
   
   //output format(does nothing)
   //#[arg(short, long)]
   //format: String,
}

fn main() {
    //let start = Instant::now();
    let args = Args::parse();
    decode(&args.source);
}

//get the frame and verify frame is HDR and other prepratory work
fn decode(
    source: &Path
) {
    let source = VapoursynthDecoder::new_from_script(source).unwrap();
    let source_frame_count = source.get_frame_count().unwrap();
    println!("frames: {} \n", source_frame_count);
    //gets the details
    let source_info = source.get_video_details();

    //get vid size
    let src_ss = source_info
        .chroma_sampling
        .get_decimation()
        .unwrap_or((0, 0));
    
    //define source config
    let src_config = YuvConfig {
        bit_depth: source_info.bit_depth as u8,
        subsampling_x: src_ss.0 as u8,
        subsampling_y: src_ss.1 as u8,
        full_range: true, // don't hardcode
        matrix_coefficients: MatrixCoefficients::BT2020NonConstantLuminance, //TODO
        transfer_characteristics: TransferCharacteristic::HybridLogGamma, //TODO
        color_primaries: ColorPrimaries::BT2020, //TODO
        //matrix_coefficients: MatrixCoefficients::BT709, //TODO
        //transfer_characteristics: TransferCharacteristic::BT1886, //TODO
        //color_primaries: ColorPrimaries::BT709, //TODO
    };

    //TODO: loop by frame, maybe use concurrency
    to_rgb::<u16>(source, src_config);
}

//conversion
fn to_rgb<T: Pixel>(
    mut yuv: VapoursynthDecoder,
    yuvconf: YuvConfig,
) {
    let start = Instant::now();

    let vido = yuv.read_specific_frame::<T>(60); //decode the frame
    let decode = start.elapsed();
    println!("Time spent reading frame: {:?} \n", decode);

    let yuvstart = Instant::now();
    let vid = vido.unwrap();
    let pix = Yuv::new(vid, yuvconf).unwrap(); // IS now a YUV frame
    let yuvconv = yuvstart.elapsed();
    println!("Time spent convert to yuv: {:?}", yuvconv);
    
    // MAKE THIS GPU
    let rgbstart = Instant::now();
    let pix1 = pix.clone();
    let rgb = Rgb::try_from(pix1).unwrap();
    let rgbd = rgb.data();
    println!("RGB \n{:?}", rgbd[0]);


    let rgbl = LinearRgb::try_from(pix).unwrap(); //convert to linearRGB
    let v = rgbl.data();
    println!("RGB L \n{:?}", v[0]);

    //let rgbdata pix.data();
    //let rgb = Rgb::new(pix, pix.width(), pix.height(), yuvconf.transfer_characteristics, yuvconf.color_primaries);
    let rgbconv = rgbstart.elapsed();
    // MAKE THIS GPU

    println!("Time spent coverting to rgb: {:?}", rgbconv);

    let hslstart = Instant::now();
    let hsl = Hsl::from(rgbl); //convert to Hsl
    let hslconv = hslstart.elapsed();
    println!("Time spent coverting to hsl: {:?}", hslconv);


    let finish = start.elapsed();
    println!("Time elapsed in rgb() is: {:?} \n", finish);

    luma(hsl);
}

fn luma(
    img: Hsl
) {
    let start = Instant::now();
    let imgdata = img.data();
    //HSL we need L
    let finish = start.elapsed();
    println!("Time elapsed in luma() is: {:?}", finish);
    let v = imgdata;

    //let values: Vec<f32>= v.iter().map(|x| x[2]).collect();
    //println!("{:?}", values)
    println!("{:#?}", v[1]);
    //let sum: f32 = values.iter().sum();
    //let len = values.len();
    //let average = sum as f32 / len as f32;
    //println!("average: {}", average);
//
    //let mut p95v = values.clone();
    //p95v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //let index = (len as f32 * 0.95) as usize;
    //let p95 = p95v[index];
    //println!("95th: {}", p95);
//
    //let index2 = (len as f32 * 0.99) as usize;
    //let p99 = p95v[index2];
    //println!("99th: {}", p99);
}

//convert to hsl
//fn to_hsl(
//    //rgb:
//) {
//    //nothing here
//}
////L channel is luminance
//fn luma(
//    //hsl:
//) {
//    //nothing here
//}