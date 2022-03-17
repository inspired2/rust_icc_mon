# rust_icc_mon
rust iccp analyzer&amp;converter

tested to work under windows 10

this software intends to be used in digital photoprinting companies which run NORITSU QSS & Fuji digital minilabs for
pre-print processing images that cannot be printed directly due to corrupt or invalid images' ICC profiles (some minilabs ignore such errors but print images with deviated colors);

use "cargo build --release" to get binary executable
just put it in the folder with jpeg images and run;
it will check if image contains valid IEC sRGB ICC profile;
it will convert bad profiles (including corrupted ones and google's truncated profiles) to standart IEC sRGB profile;
this will happen to all images on the same level and recursively for all subfolders;
images that are to be converted will be overwritten - use with caution;
