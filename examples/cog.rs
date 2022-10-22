use std::fs::File;
use tiff::decoder::{Decoder, DecodingResult};
use tiff::tags::Tag;
use tiff::ColorType;

fn read_cog() {
    // https://gdal.org/drivers/raster/cog.html
    let img_file = File::open("imagery/seen.tif").expect("Cannot find test image!");
    let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
    // Decoder {
    //     reader: SmartReader {
    //         reader: File {
    //             fd: 3,
    //             path: "imagery/seen.tif",
    //             read: true,
    //             write: false,
    //         },
    //         byte_order: LittleEndian,
    //     },
    //     bigtiff: false,
    //     limits: Limits {
    //         decoding_buffer_size: 268435456,
    //         ifd_value_size: 1048576,
    //         intermediate_buffer_size: 134217728,
    //         _non_exhaustive: (),
    //     },
    //     current_chunk: 0,
    //     next_ifd: Some(
    //         562,
    //     ),
    //     ifd_offsets: [
    //         192,
    //         562,
    //     ],
    //     seen_ifds: {
    //         192,
    //         562,
    //     },
    //     image: Image {
    //         ifd: Some(
    //             {
    //                 TileByteCounts: Entry { type_: LONG, count: 8, offset: [174, 3, 0, 0, 0, 0, 0, 0] },
    //                 ModelPixelScaleTag: Entry { type_: DOUBLE, count: 3, offset: [146, 1, 0, 0, 0, 0, 0, 0] },
    //                 TileOffsets: Entry { type_: LONG, count: 8, offset: [142, 3, 0, 0, 0, 0, 0, 0] },
    //                 SampleFormat: Entry { type_: SHORT, count: 3, offset: [140, 1, 0, 0, 0, 0, 0, 0] },
    //                 ModelTiepointTag: Entry { type_: DOUBLE, count: 6, offset: [170, 1, 0, 0, 0, 0, 0, 0] },
    //                 GeoKeyDirectoryTag: Entry { type_: SHORT, count: 32, offset: [218, 1, 0, 0, 0, 0, 0, 0] },
    //                 TileLength: Entry { type_: SHORT, count: 1, offset: [0, 2, 0, 0, 0, 0, 0, 0] },
    //                 ImageWidth: Entry { type_: SHORT, count: 1, offset: [25, 6, 0, 0, 0, 0, 0, 0] },
    //                 GeoAsciiParamsTag: Entry { type_: ASCII, count: 24, offset: [26, 2, 0, 0, 0, 0, 0, 0] },
    //                 TileWidth: Entry { type_: SHORT, count: 1, offset: [0, 2, 0, 0, 0, 0, 0, 0] },
    //                 Compression: Entry { type_: SHORT, count: 1, offset: [1, 0, 0, 0, 0, 0, 0, 0] },
    //                 PlanarConfiguration: Entry { type_: SHORT, count: 1, offset: [1, 0, 0, 0, 0, 0, 0, 0] },
    //                 SamplesPerPixel: Entry { type_: SHORT, count: 1, offset: [3, 0, 0, 0, 0, 0, 0, 0] },
    //                 ImageLength: Entry { type_: SHORT, count: 1, offset: [63, 3, 0, 0, 0, 0, 0, 0] },
    //                 BitsPerSample: Entry { type_: SHORT, count: 3, offset: [134, 1, 0, 0, 0, 0, 0, 0] },
    //                 PhotometricInterpretation: Entry { type_: SHORT, count: 1, offset: [2, 0, 0, 0, 0, 0, 0, 0] },
    //             },
    //         ),
    //         width: 1561,
    //         height: 831,
    //         bits_per_sample: [
    //             8,
    //             8,
    //             8,
    //         ],
    //         samples: 3,
    //         sample_format: [
    //             Uint,
    //             Uint,
    //             Uint,
    //         ],
    //         photometric_interpretation: RGB,
    //         compression_method: None,
    //         predictor: None,
    //         jpeg_tables: None,
    //         chunk_type: Tile,
    //         strip_decoder: None,
    //         tile_attributes: Some(
    //             TileAttributes {
    //                 image_width: 1561,
    //                 image_height: 831,
    //                 tile_width: 512,
    //                 tile_length: 512,
    //             },
    //         ),
    //         chunk_offsets: [
    //             2360314,
    //             3146754,
    //             3933194,
    //             4719634,
    //             5506074,
    //             6292514,
    //             7078954,
    //             7865394,
    //         ],
    //         chunk_bytes: [
    //             786432,
    //             786432,
    //             786432,
    //             786432,
    //             786432,
    //             786432,
    //             786432,
    //             786432,
    //         ],
    //     }
    // }
    dbg!(&decoder);
    dbg!(decoder.dimensions().unwrap());
    if let Ok(geokeys) = decoder.get_tag_u32_vec(Tag::GeoKeyDirectoryTag) {
        dbg!(geokeys);
    }
    if let Ok(geo_params) = decoder.get_tag_ascii_string(Tag::GeoAsciiParamsTag) {
        dbg!(geo_params);
    }
    if let Ok(model_tiepoint) = decoder.get_tag_f64_vec(Tag::ModelTiepointTag) {
        dbg!(model_tiepoint);
    }
    if let Ok(model_pixel_scale) = decoder.get_tag_f64_vec(Tag::ModelPixelScaleTag) {
        dbg!(model_pixel_scale);
    }

    assert_eq!(decoder.colortype().unwrap(), ColorType::RGB(8));

    let tiles = decoder.tile_count().unwrap();
    dbg!(tiles);
    dbg!(decoder.chunk_dimensions());

    for tile in 0..tiles {
        // tiles in row major order
        dbg!(decoder.chunk_data_dimensions(tile));
        match decoder.read_chunk(tile).unwrap() {
            DecodingResult::U8(res) => {
                let _sum: u64 = res.into_iter().map(<u64>::from).sum();
            }
            _ => panic!("Wrong bit depth"),
        }
    }

    while decoder.more_images() {
        decoder.next_image().unwrap();
        dbg!(decoder.dimensions().unwrap());
        if let Ok(subfile_type) = decoder.get_tag_u64(Tag::NewSubfileType) {
            dbg!(subfile_type);
        }
    }
}

fn main() {
    read_cog();
}
