use crate::bindings::*;

use std::ffi::{CStr, CString};

pub(crate) fn version() -> &'static str {
    unsafe { CStr::from_ptr(ncnn_version()) }.to_str().unwrap()
}

pub(crate) struct Net(*mut __ncnn_net_t);

impl Net {
    pub(crate) fn new() -> Self {
        Net {
            0: unsafe { ncnn_net_create() },
        }
    }

    pub(crate) fn load_param(&mut self, path: &str) -> Result<(), i32> {
        let path = CString::new(path).unwrap();
        let errno = unsafe { ncnn_net_load_param(self.0, path.as_ptr()) };
        if errno == 0 {
            Ok(())
        } else {
            Err(errno)
        }
    }

    pub(crate) fn load_model(&mut self, path: &str) -> Result<(), i32> {
        let path = CString::new(path).unwrap();
        let errno = unsafe { ncnn_net_load_model(self.0, path.as_ptr()) };
        if errno == 0 {
            Ok(())
        } else {
            Err(errno)
        }
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe { ncnn_net_clear(self.0) };
        // unsafe { ncnn_net_destroy(self.0) };
    }
}

/*
input and output
ncnn Mat is the data structure for input and output data

Input image should be converted to Mat, and subtracted mean values and normalized when needed

#include "mat.h"
unsigned char* rgbdata;// data pointer to RGB image pixels
int w;// image width
int h;// image height
ncnn::Mat in = ncnn::Mat::from_pixels(rgbdata, ncnn::Mat::PIXEL_RGB, w, h);

const float mean_vals[3] = {104.f, 117.f, 123.f};
in.substract_mean_normalize(mean_vals, 0);


Execute the network inference and retrieve the result

#include "net.h"
ncnn::Mat in;// input blob as above
ncnn::Mat out;
ncnn::Extractor ex = net.create_extractor();
ex.set_light_mode(true);
ex.input("data", in);
ex.extract("prob", out);


If you load model with binary param.bin file, you should use the enum value in alexnet.id.h file instead of the blob name

#include "net.h"
#include "alexnet.id.h"
ncnn::Mat in;// input blob as above
ncnn::Mat out;
ncnn::Extractor ex = net.create_extractor();
ex.set_light_mode(true);
ex.input(alexnet_param_id::BLOB_data, in);
ex.extract(alexnet_param_id::BLOB_prob, out);


Read the data in the output Mat. Iterate data to get all classification scores.

ncnn::Mat out_flatterned = out.reshape(out.w * out.h * out.c);
std::vector<float> scores;
scores.resize(out_flatterned.w);
for (int j=0; j<out_flatterned.w; j++)
{
    scores[j] = out_flatterned[j];
}


some tricks
Set multithreading thread number with Extractor

ex.set_num_threads(4);
Convert image colorspace and resize image with Mat convenient function, these functions are well optimized

Support RGB2GRAY GRAY2RGB RGB2BGR etc, support scale up and scale down

#include "mat.h"
unsigned char* rgbdata;// data pointer to RGB image pixels
int w;// image width
int h;// image height
int target_width = 227;// target resized width
int target_height = 227;// target resized height
ncnn::Mat in = ncnn::Mat::from_pixels_resize(rgbdata, ncnn::Mat::PIXEL_RGB2GRAY, w, h, target_width, target_height);
 */
