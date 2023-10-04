use halo2curves::{bn256::Fr as F, serde::SerdeObject};
pub fn half_square(k:u64) -> F {
    match k {
       0 => F::from_raw_bytes_unchecked(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
       1 => F::from_raw_bytes_unchecked(&[255, 255, 255, 15, 108, 10, 30, 188, 110, 143, 70, 134, 183, 23, 204, 215, 162, 167, 126, 126, 73, 186, 175, 71, 214, 95, 206, 30, 141, 177, 155, 15]),
       2 => F::from_raw_bytes_unchecked(&[251, 255, 255, 79, 28, 52, 150, 172, 41, 205, 96, 159, 149, 118, 252, 54, 46, 70, 121, 120, 111, 163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14]),
       3 => F::from_raw_bytes_unchecked(&[245, 255, 255, 175, 164, 114, 74, 21, 194, 41, 8, 197, 226, 4, 197, 69, 255, 51, 113, 111, 40, 1, 141, 20, 53, 30, 222, 82, 16, 161, 176, 43]),
       4 => F::from_raw_bytes_unchecked(&[235, 255, 255, 79, 221, 218, 118, 110, 21, 196, 201, 3, 14, 242, 189, 179, 91, 192, 99, 96, 7, 72, 106, 225, 147, 220, 237, 134, 147, 144, 197, 7]),
       5 => F::from_raw_bytes_unchecked(&[223, 255, 255, 15, 238, 87, 223, 63, 70, 125, 24, 79, 168, 14, 79, 209, 253, 155, 83, 78, 121, 3, 167, 61, 159, 90, 154, 248, 48, 227, 17, 3]),
       6 => F::from_raw_bytes_unchecked(&[209, 255, 255, 239, 214, 233, 131, 137, 84, 85, 244, 166, 177, 90, 120, 158, 229, 198, 64, 57, 126, 51, 67, 41, 87, 152, 227, 167, 232, 152, 149, 29]),
       7 => F::from_raw_bytes_unchecked(&[192, 255, 255, 255, 3, 155, 130, 7, 175, 219, 163, 145, 225, 237, 5, 243, 181, 232, 169, 159, 95, 146, 238, 235, 145, 245, 151, 179, 71, 99, 236, 38]),
       8 => F::from_raw_bytes_unchecked(&[172, 255, 255, 63, 117, 107, 219, 185, 85, 16, 39, 15, 56, 200, 247, 206, 110, 1, 143, 129, 29, 32, 169, 133, 79, 114, 183, 27, 78, 66, 22, 31]),
       9 => F::from_raw_bytes_unchecked(&[149, 255, 255, 175, 42, 91, 142, 160, 72, 243, 125, 31, 181, 233, 77, 50, 16, 17, 240, 222, 183, 220, 114, 246, 143, 14, 66, 224, 251, 53, 19, 6]),
       10 => F::from_raw_bytes_unchecked(&[124, 255, 255, 63, 184, 95, 125, 255, 24, 245, 97, 60, 161, 58, 60, 69, 247, 111, 78, 57, 229, 13, 156, 246, 124, 106, 105, 226, 195, 140, 71, 12]),
       11 => F::from_raw_bytes_unchecked(&[96, 255, 255, 255, 137, 131, 198, 146, 53, 165, 25, 236, 179, 210, 142, 223, 198, 197, 40, 15, 239, 109, 212, 205, 236, 229, 251, 64, 51, 248, 78, 1]),
       12 => F::from_raw_bytes_unchecked(&[66, 255, 255, 223, 51, 188, 75, 158, 47, 116, 94, 168, 53, 154, 121, 41, 220, 106, 0, 226, 139, 66, 108, 52, 9, 33, 43, 221, 188, 198, 141, 21]),
       13 => F::from_raw_bytes_unchecked(&[33, 255, 255, 239, 33, 20, 43, 222, 117, 241, 118, 247, 221, 168, 200, 250, 217, 6, 84, 48, 5, 70, 19, 114, 168, 123, 197, 213, 237, 169, 159, 24]),
       14 => F::from_raw_bytes_unchecked(&[253, 254, 255, 47, 84, 139, 100, 82, 8, 29, 99, 217, 172, 254, 123, 83, 192, 153, 35, 250, 90, 120, 201, 134, 202, 245, 202, 42, 198, 161, 132, 10]),
       15 => F::from_raw_bytes_unchecked(&[215, 254, 255, 143, 94, 23, 218, 62, 120, 103, 220, 199, 234, 131, 199, 91, 236, 123, 240, 192, 67, 31, 223, 42, 153, 47, 109, 189, 184, 252, 160, 27]),
       16 => F::from_raw_bytes_unchecked(&[174, 254, 255, 31, 173, 194, 169, 95, 52, 96, 41, 73, 79, 80, 119, 235, 0, 85, 57, 3, 9, 245, 3, 166, 234, 136, 122, 172, 82, 108, 144, 27]),
       17 => F::from_raw_bytes_unchecked(&[130, 254, 255, 223, 63, 141, 211, 180, 60, 7, 74, 93, 218, 99, 139, 2, 254, 36, 254, 192, 170, 249, 55, 248, 190, 1, 243, 247, 147, 240, 82, 10]),
       18 => F::from_raw_bytes_unchecked(&[84, 254, 255, 191, 170, 108, 57, 130, 34, 205, 247, 125, 212, 166, 55, 201, 64, 68, 192, 123, 223, 114, 203, 217, 63, 58, 8, 129, 239, 215, 76, 24]),
       19 => F::from_raw_bytes_unchecked(&[35, 254, 255, 207, 89, 107, 249, 131, 84, 65, 121, 49, 245, 48, 72, 23, 108, 90, 254, 177, 240, 26, 110, 146, 67, 146, 136, 102, 242, 211, 25, 21]),
       20 => F::from_raw_bytes_unchecked(&[239, 253, 255, 15, 77, 137, 19, 186, 210, 99, 206, 119, 60, 2, 189, 236, 127, 103, 184, 99, 222, 241, 31, 34, 202, 9, 116, 168, 156, 228, 185, 0]),
       21 => F::from_raw_bytes_unchecked(&[185, 253, 255, 111, 24, 188, 105, 104, 46, 165, 176, 202, 242, 2, 202, 113, 217, 195, 111, 18, 95, 61, 49, 65, 253, 64, 252, 39, 97, 88, 145, 11]),
       22 => F::from_raw_bytes_unchecked(&[128, 253, 255, 255, 39, 14, 26, 75, 214, 148, 102, 176, 207, 74, 59, 126, 27, 23, 163, 60, 188, 183, 81, 55, 179, 151, 239, 3, 205, 224, 59, 5]),
       23 => F::from_raw_bytes_unchecked(&[69, 253, 255, 175, 15, 117, 6, 166, 91, 163, 169, 162, 27, 194, 68, 58, 163, 185, 211, 99, 172, 166, 209, 188, 21, 174, 127, 29, 83, 204, 29, 30]),
       24 => F::from_raw_bytes_unchecked(&[7, 253, 255, 143, 59, 251, 76, 53, 45, 96, 192, 39, 142, 128, 178, 125, 19, 83, 128, 6, 121, 196, 96, 25, 251, 227, 122, 147, 128, 204, 210, 37]),
       25 => F::from_raw_bytes_unchecked(&[198, 252, 255, 159, 171, 160, 237, 248, 74, 203, 170, 63, 39, 134, 132, 72, 108, 227, 168, 36, 34, 17, 255, 76, 99, 57, 225, 101, 85, 225, 90, 28]),
       26 => F::from_raw_bytes_unchecked(&[130, 252, 255, 223, 95, 101, 232, 240, 180, 228, 104, 234, 230, 210, 186, 154, 173, 106, 77, 190, 167, 140, 172, 87, 78, 174, 178, 148, 209, 10, 182, 1]),
       27 => F::from_raw_bytes_unchecked(&[60, 252, 255, 63, 236, 62, 31, 97, 252, 28, 180, 161, 21, 79, 137, 156, 52, 65, 239, 84, 192, 124, 185, 241, 229, 226, 32, 1, 104, 151, 72, 6]),
       28 => F::from_raw_bytes_unchecked(&[244, 251, 255, 191, 80, 45, 146, 73, 33, 116, 140, 101, 179, 250, 239, 77, 1, 103, 142, 232, 107, 225, 37, 27, 42, 215, 43, 171, 24, 135, 18, 42]),
       29 => F::from_raw_bytes_unchecked(&[168, 251, 255, 127, 101, 69, 125, 34, 1, 9, 127, 66, 47, 5, 135, 94, 89, 43, 40, 118, 61, 47, 81, 99, 199, 74, 112, 208, 253, 60, 75, 12]),
       30 => F::from_raw_bytes_unchecked(&[90, 251, 255, 95, 82, 114, 164, 115, 190, 188, 254, 43, 26, 63, 182, 30, 247, 62, 191, 0, 162, 241, 219, 58, 17, 126, 81, 51, 253, 85, 187, 13]),
       31 => F::from_raw_bytes_unchecked(&[10, 251, 255, 95, 23, 180, 7, 61, 89, 143, 11, 34, 116, 168, 125, 142, 218, 161, 83, 136, 153, 40, 198, 161, 7, 113, 207, 211, 22, 210, 98, 46]),
       32 => F::from_raw_bytes_unchecked(&[182, 250, 255, 159, 140, 31, 227, 246, 174, 159, 50, 49, 172, 112, 117, 93, 73, 163, 226, 9, 183, 72, 111, 39, 87, 227, 134, 239, 100, 20, 121, 13]),
       33 => F::from_raw_bytes_unchecked(&[96, 250, 255, 255, 217, 159, 250, 40, 226, 206, 230, 76, 83, 104, 5, 220, 253, 243, 110, 136, 103, 221, 119, 60, 83, 21, 219, 72, 205, 185, 198, 11]),
       34 => F::from_raw_bytes_unchecked(&[8, 250, 255, 127, 255, 52, 78, 211, 242, 28, 40, 117, 105, 143, 45, 10, 248, 147, 248, 3, 171, 230, 223, 224, 251, 6, 204, 223, 79, 194, 75, 41]),
       35 => F::from_raw_bytes_unchecked(&[172, 249, 255, 63, 213, 243, 25, 110, 190, 168, 131, 182, 93, 21, 134, 151, 125, 210, 124, 121, 20, 217, 6, 164, 253, 119, 246, 241, 6, 145, 63, 5]),
       36 => F::from_raw_bytes_unchecked(&[78, 249, 255, 31, 131, 199, 33, 129, 103, 83, 108, 4, 193, 202, 118, 212, 72, 96, 254, 235, 16, 64, 141, 246, 171, 168, 189, 65, 216, 194, 106, 0]),
       37 => F::from_raw_bytes_unchecked(&[238, 248, 255, 31, 9, 176, 101, 12, 238, 28, 226, 94, 147, 175, 255, 192, 89, 61, 125, 91, 160, 27, 115, 216, 6, 153, 33, 207, 195, 87, 205, 26]),
       38 => F::from_raw_bytes_unchecked(&[139, 248, 255, 79, 211, 183, 3, 204, 192, 148, 43, 76, 140, 219, 236, 52, 83, 17, 120, 70, 12, 38, 104, 145, 228, 168, 240, 184, 86, 1, 3, 36]),
       39 => F::from_raw_bytes_unchecked(&[37, 248, 255, 175, 225, 222, 251, 191, 223, 186, 72, 204, 171, 78, 62, 48, 53, 220, 238, 172, 84, 95, 108, 33, 69, 216, 42, 255, 144, 191, 11, 28]),
       40 => F::from_raw_bytes_unchecked(&[188, 247, 255, 63, 52, 37, 78, 232, 74, 143, 57, 223, 241, 8, 244, 178, 255, 157, 225, 142, 121, 199, 127, 136, 40, 39, 208, 161, 114, 146, 231, 2]),
       41 => F::from_raw_bytes_unchecked(&[81, 247, 255, 239, 94, 128, 220, 136, 147, 130, 183, 254, 166, 242, 65, 229, 15, 175, 209, 109, 49, 164, 242, 126, 184, 53, 18, 130, 110, 200, 250, 8]),
       42 => F::from_raw_bytes_unchecked(&[228, 246, 255, 191, 97, 240, 166, 161, 185, 148, 194, 42, 203, 11, 40, 199, 101, 15, 191, 73, 124, 245, 196, 4, 245, 3, 241, 159, 132, 97, 69, 46]),
       43 => F::from_raw_bytes_unchecked(&[115, 246, 255, 207, 20, 138, 233, 170, 154, 228, 231, 111, 205, 131, 62, 8, 71, 14, 167, 31, 237, 47, 86, 169, 138, 81, 9, 57, 207, 192, 254, 17]),
       44 => F::from_raw_bytes_unchecked(&[0, 246, 255, 255, 159, 56, 104, 44, 89, 83, 154, 193, 62, 43, 237, 248, 109, 92, 140, 242, 240, 222, 70, 221, 204, 94, 190, 15, 52, 131, 239, 20]),
       45 => F::from_raw_bytes_unchecked(&[138, 245, 255, 95, 111, 6, 65, 226, 99, 112, 32, 166, 214, 25, 0, 113, 125, 161, 237, 64, 209, 188, 70, 232, 145, 139, 222, 66, 64, 90, 179, 6]),
       46 => F::from_raw_bytes_unchecked(&[18, 245, 255, 223, 22, 233, 85, 16, 76, 172, 51, 151, 221, 55, 171, 152, 210, 53, 76, 140, 68, 15, 166, 130, 3, 120, 155, 179, 102, 148, 174, 23]),
       47 => F::from_raw_bytes_unchecked(&[151, 244, 255, 143, 2, 235, 196, 114, 128, 150, 26, 27, 11, 157, 186, 71, 16, 193, 38, 83, 148, 144, 20, 244, 247, 131, 195, 128, 52, 227, 124, 23]),
       48 => F::from_raw_bytes_unchecked(&[25, 244, 255, 111, 50, 12, 142, 9, 1, 47, 213, 49, 95, 73, 46, 126, 54, 67, 125, 149, 192, 64, 146, 60, 111, 175, 86, 170, 169, 70, 30, 6]),
       49 => F::from_raw_bytes_unchecked(&[153, 243, 255, 111, 58, 66, 147, 24, 95, 230, 28, 85, 34, 37, 58, 100, 162, 20, 209, 212, 127, 101, 111, 20, 147, 154, 134, 17, 57, 13, 247, 19]),
        _ => panic!(),
    }
}
