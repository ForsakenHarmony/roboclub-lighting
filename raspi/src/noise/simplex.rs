use super::util::fast_floor;

const GRAD3: [[i32; 3]; 12] = [
	[1, 1, 0],
	[-1, 1, 0],
	[1, -1, 0],
	[-1, -1, 0],
	[1, 0, 1],
	[-1, 0, 1],
	[1, 0, -1],
	[-1, 0, -1],
	[0, 1, 1],
	[0, -1, 1],
	[0, 1, -1],
	[0, -1, -1],
];

// const P: [usize; 256] = [151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180];

const PERM: [usize; 512] = [
	151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
	142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
	203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
	74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
	220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
	132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
	186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
	59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
	70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
	178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
	241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
	176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
	128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
	233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
	75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
	20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
	111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
	63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
	159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
	118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
	213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
	19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
	238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
	181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
	222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

// lazy_static! {
//  static ref PERM: [usize; 512] = {
//     let mut perm = [0; 512];
//     for i in 0..512 {
//       perm[i] = P[i & 255];
//     }
//     perm
//  };
// }

fn dot(g: [i32; 3], x: f32, y: f32, z: f32) -> f32 {
	g[0] as f32 * x + g[1] as f32 * y + g[2] as f32 * z
}

// 3D simplex noise
pub fn simplex3d(xin: f32, yin: f32, zin: f32) -> f32 {
	// Skew the input space to determine which simplex cell we're in
	let f3 = 1.0 / 3.0;
	let s = (xin + yin + zin) * f3; // Very nice and simple skew factor for 3D
	let i = fast_floor(xin + s) as f32;
	let j = fast_floor(yin + s) as f32;
	let k = fast_floor(zin + s) as f32;

	let g3 = 1.0 / 6.0; // Very nice and simple unskew factor, too
	let t = (i + j + k) * g3;
	let x0_ = i - t; // Unskew the cell origin back to (x,y,z) space
	let y0_ = j - t;
	let z0_ = k - t;
	let x0 = xin - x0_; // The x,y,z distances from the cell origin
	let y0 = yin - y0_;
	let z0 = zin - z0_;

	// For the 3D case, the simplex shape is a slightly irregular tetrahedron.
	// Determine which simplex we are in.
	// Offsets for second corner of simplex in (i,j,k) coords
	let i1;
	let j1;
	let k1;
	// Offsets for third corner of simplex in (i,j,k) coords
	let i2;
	let j2;
	let k2;

	if x0 >= y0 {
		if y0 >= z0 {
			// X Y Z order
			i1 = 1;
			j1 = 0;
			k1 = 0;
			i2 = 1;
			j2 = 1;
			k2 = 0;
		} else if x0 >= z0 {
			// X Z Y order
			i1 = 1;
			j1 = 0;
			k1 = 0;
			i2 = 1;
			j2 = 0;
			k2 = 1;
		} else {
			// Z X Y order
			i1 = 0;
			j1 = 0;
			k1 = 1;
			i2 = 1;
			j2 = 0;
			k2 = 1;
		}
	// x0<y0
	} else {
		// Z Y X order
		if y0 < z0 {
			i1 = 0;
			j1 = 0;
			k1 = 1;
			i2 = 0;
			j2 = 1;
			k2 = 1;
		}
		// Y Z X order
		else if x0 < z0 {
			i1 = 0;
			j1 = 1;
			k1 = 0;
			i2 = 0;
			j2 = 1;
			k2 = 1;
		}
		// Y X Z order
		else {
			i1 = 0;
			j1 = 1;
			k1 = 0;
			i2 = 1;
			j2 = 1;
			k2 = 0;
		}
	}

	// A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
	// a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
	// a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
	// c = 1/6.
	let x1 = x0 - i1 as f32 + g3; // Offsets for second corner in (x,y,z) coords
	let y1 = y0 - j1 as f32 + g3;
	let z1 = z0 - k1 as f32 + g3;
	let x2 = x0 - i2 as f32 + 2.0 * g3; // Offsets for third corner in (x,y,z) coords
	let y2 = y0 - j2 as f32 + 2.0 * g3;
	let z2 = z0 - k2 as f32 + 2.0 * g3;
	let x3 = x0 - 1.0 + 3.0 * g3; // Offsets for last corner in (x,y,z) coords
	let y3 = y0 - 1.0 + 3.0 * g3;
	let z3 = z0 - 1.0 + 3.0 * g3;

	// Work out the hashed gradient indices of the four simplex corners
	let ii = i as usize & 255;
	let jj = j as usize & 255;
	let kk = k as usize & 255;
	let gi0 = PERM[ii + PERM[jj + PERM[kk]]] % 12;
	let gi1 = PERM[ii + i1 + PERM[jj + j1 + PERM[kk + k1]]] % 12;
	let gi2 = PERM[ii + i2 + PERM[jj + j2 + PERM[kk + k2]]] % 12;
	let gi3 = PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1]]] % 12;

	// Calculate the contribution from the four corners
	let mut t0 = 0.5 - x0 * x0 - y0 * y0 - z0 * z0;
	let n0 = if t0 < 0.0 {
		0.0
	} else {
		t0 *= t0;
		t0 * t0 * dot(GRAD3[gi0], x0, y0, z0)
	};
	let mut t1 = 0.5 - x1 * x1 - y1 * y1 - z1 * z1;
	let n1 = if t1 < 0.0 {
		0.0
	} else {
		t1 *= t1;
		t1 * t1 * dot(GRAD3[gi1], x1, y1, z1)
	};
	let mut t2 = 0.5 - x2 * x2 - y2 * y2 - z2 * z2;
	let n2 = if t2 < 0.0 {
		0.0
	} else {
		t2 *= t2;
		t2 * t2 * dot(GRAD3[gi2], x2, y2, z2)
	};
	let mut t3 = 0.5 - x3 * x3 - y3 * y3 - z3 * z3;
	let n3 = if t3 < 0.0 {
		0.0
	} else {
		t3 *= t3;
		t3 * t3 * dot(GRAD3[gi3], x3, y3, z3)
	};

	// Add contributions from each corner to get the final noise value.
	// The result is scaled to stay just inside [-1,1]
	32.0 * (n0 + n1 + n2 + n3)
}
