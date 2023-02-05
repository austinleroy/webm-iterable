use ebml_iterable::tools::{self as ebml_tools, Vint};

use crate::{matroska_spec::{Frame, BlockLacing}, errors::WebmCoercionError};

#[inline(always)]
pub fn read_frame_data<'a>(frame_data: &'a [u8], lacing: &Option<BlockLacing>) -> Result<Vec<Frame<'a>>, WebmCoercionError> {
    if let Some(lacing) = lacing {
        let frame_count = frame_data[0] as usize + 1;
        let (mut frame_start, sizes) = match lacing {
            BlockLacing::Xiph => {
                let mut sizes: Vec<usize> = Vec::with_capacity(frame_count-1);
                let mut position: usize = 1;
                let mut size = 0;
                while sizes.len() < frame_count - 1 {
                    size += frame_data[position] as usize;
                    if frame_data[position] != 0xFF {
                        sizes.push(size);
                        size = 0;
                    }
                    position += 1;
                }
                Ok((position, sizes))
            },
            BlockLacing::Ebml => {
                let mut sizes: Vec<usize> = Vec::with_capacity(frame_count-1);
                let mut position: usize = 1;
                while sizes.len() < frame_count - 1 {
                    if let Some((val, val_len)) = ebml_tools::read_vint(&frame_data[position..]).ok().flatten() {
                        if let Some(last) = sizes.last() {
                            // This reads the value in two's complement notation like the spec describes
                            // let difference = if val > ((1 << ((7 * val_len) - 1)) - 1) {
                            //     val | !((1 << (7 * val_len)) - 1)
                            // } else {
                            //     val
                            // } as i64;

                            // But the spec example just subtracts half the range like this
                            let difference = (val as i64) - ((1 << ((7 * val_len) - 1)) - 1);

                            // I've opened up an issue with the specification: https://github.com/ietf-wg-cellar/matroska-specification/issues/726
                            // In the mean time, example files with EBML Lacing seem to use the "subtract half range" approach, so we'll make
                            // the assumption to go with that until there's an update otherwise.

                            sizes.push((difference + (*last as i64)) as usize);
                        } else {
                            sizes.push(val as usize);
                        }
                        position += val_len;
                    } else {
                        return Err(WebmCoercionError::BlockCoercionError(String::from("Unable to read ebml lacing frame sizes in block")));
                    }
                }

                Ok((position, sizes))
            },
            BlockLacing::FixedSize => {
                let total_size = frame_data.len() - 1;
                if total_size % frame_count == 0 {
                    let frame_size = total_size / frame_count;
                    Ok((1usize, vec![frame_size; frame_count - 1]))
                } else {
                    Err(WebmCoercionError::BlockCoercionError(String::from("Block frame count with fixed lacing size did not match frame data length")))
                }
            }
        }?;

        let mut frames: Vec<Frame> = Vec::with_capacity(frame_count);
        for size in sizes {
            frames.push(Frame { data: &frame_data[frame_start..(frame_start+size)] });
            frame_start += size;
        }
        frames.push(Frame { data: &frame_data[frame_start..] });

        Ok(frames)
    } else {
        Ok(vec![Frame { data: frame_data }])
    }
}

#[inline(always)]
pub fn write_frame_data(frames: &Vec<Frame>, mut desired_lacing: Option<BlockLacing>) -> (Vec<u8>, Option<BlockLacing>) {
    if frames.len() == 1 {
        // If there is only 1 frame, lacing doesn't apply
       desired_lacing = None;
    } else if desired_lacing.is_none() {
        // If there is more than 1 frame and lacing is not set, default to Ebml lacing
        desired_lacing = Some(BlockLacing::Ebml);
    }

    if let Some(lacing) = desired_lacing {
        let sizes = match lacing {
            BlockLacing::Xiph => {
                let mut sizes: Vec<u8> = Vec::new();
                for frame in &frames[..frames.len()-1] {
                    sizes.resize(sizes.len() + frame.data.len()/255, 0xFF);
                    sizes.push((frame.data.len()%255) as u8);
                }
                sizes
            },
            BlockLacing::Ebml => {
                let mut last_size: Option<usize> = None;
                let mut sizes: Vec<u8> = Vec::new();
                for frame in &frames[..frames.len()-1] {
                    let size = frame.data.len();
                    let written_size = if let Some(last_size) = last_size {
                        // Just like the issue in parsing EBML lacing, this writes the value in two's complement notation like the spec describes
                        // let mut diff = (size as i64) - (last_size as i64);
                        // if diff < 0 {
                        //     let mut length: usize = 1;
                        //     while length <= 8 {
                        //         if diff > -(1 << ((7 * length) - 1)) {
                        //             break;
                        //         }
                        //         length += 1;
                        //     }
                        //     diff &= (1 << (7 * length)) - 1;

                        // }
                        // diff as u64

                        // But the spec example would be to just add half the range like this
                        let diff = (size as i64) - (last_size as i64);
                        let mut length: usize = 1;
                        while length <= 8 {
                            if diff > -(1 << ((7 * length) - 1)) && diff < (1 << ((7 * length) - 1)) {
                                break;
                            }
                            length += 1;
                        }
                        (diff + (1 << ((7 * length) - 1)) - 1) as u64
                    } else {
                        size as u64
                    };
                    sizes.append(&mut written_size.as_vint().unwrap());
                    last_size = Some(size);
                }
                sizes
            },
            BlockLacing::FixedSize => {
                //FixedSize block lacing *cannot* be used with frames of different sizes
                assert!(frames.iter().skip(1).all(|f| f.data.len() == frames[0].data.len()));
                vec![]
            }
        };

        let mut payload: Vec<u8> = Vec::with_capacity(1 + sizes.len() + frames.iter().fold(0, |a, c| a + c.data.len()));

        payload.push((frames.len()-1) as u8);
        payload.extend_from_slice(sizes.as_slice());
        for frame in frames {
            payload.extend_from_slice(frame.data);
        }

        (payload, desired_lacing)
    } else {
        (frames[0].data.to_vec(), desired_lacing)
    }
}