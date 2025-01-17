use std::{
	fmt::Debug,
	io::ErrorKind,
	ops::{Bound, Index, IndexMut, RangeBounds},
	slice::{Iter, IterMut},
};

use eyre::{eyre, Result};
use palette::{encoding, Mix, WithAlpha};
use serial_ws2812::{Config as SerialConfig, Error, SerialWs2812};
use tracing::{error, instrument, trace};

use crate::{
	color::{Rgb, Rgba},
	config::GlobalConfig,
};

pub const LEDS_PER_STRIP: usize = 480;
pub const STRIPS: usize = 8;

pub struct Controller {
	serial: SerialWs2812,

	state:  [[Rgba; LEDS_PER_STRIP]; STRIPS],
	buffer: [u8; 3 * LEDS_PER_STRIP * STRIPS],
}

pub trait LedController {
	fn write_state(&mut self, config: &GlobalConfig);
	fn state_mut(&mut self) -> &mut [[Rgba; LEDS_PER_STRIP]; STRIPS];
	fn state_mut_flat(&mut self) -> &mut [Rgba; LEDS_PER_STRIP * STRIPS];
	fn views_mut(&mut self) -> Views;
	fn section(&mut self, strip: usize, start: usize, len: usize, reversed: bool) -> Section;
}

impl Controller {
	pub fn new() -> Result<Self> {
		let mut serial = SerialWs2812::find(SerialConfig {
			leds:   LEDS_PER_STRIP,
			strips: STRIPS,
		})?
		.ok_or(eyre!("device not found"))?;

		serial.configure()?;

		Ok(Controller {
			serial,

			state: [[(); LEDS_PER_STRIP]; STRIPS].map(|strips| strips.map(|_| Rgba::default())),
			buffer: [0u8; 3 * LEDS_PER_STRIP * STRIPS],
		})
	}

	#[instrument(skip(self))]
	fn encode_state(&mut self, config: &GlobalConfig) {
		#[allow(clippy::identity_op)]
		for (i, c) in self.state.iter().flatten().enumerate() {
			let (c, a) = c.split();
			// from black to the colour
			let c = Rgb::default().mix(c.into_linear(), a * config.brightness);

			let (r, g, b) = if config.as_srgb {
				c.into_encoding::<u8, encoding::Srgb>()
					// .into_format::<u8>()
					.into_components()
			} else {
				c.into_format::<u8>().into_components()
			};

			self.buffer[i * 3 + 0] = r;
			self.buffer[i * 3 + 1] = g;
			self.buffer[i * 3 + 2] = b;
		}
	}
}

impl LedController for Controller {
	/// Writes the inner state to the strips
	#[instrument(skip(self))]
	fn write_state(&mut self, config: &GlobalConfig) {
		trace!("sending ws2812 buffer over serial");
		self.encode_state(config);

		if let Err(e) = self.serial.send_leds(&self.buffer) {
			if let Error::IO(ref e) = e {
				if e.kind() == ErrorKind::BrokenPipe {
					panic!("broken pipe: {:#}", e);
				}
			}

			error!("error sending state: {:#}", e)
		};
	}

	#[instrument(skip(self))]
	fn state_mut(&mut self) -> &mut [[Rgba; LEDS_PER_STRIP]; STRIPS] {
		&mut self.state
	}

	#[instrument(skip(self))]
	fn state_mut_flat(&mut self) -> &mut [Rgba; LEDS_PER_STRIP * STRIPS] {
		unsafe { std::mem::transmute(&mut self.state) }
	}

	#[instrument(skip(self))]
	fn views_mut(&mut self) -> Views {
		Views::new(&mut self.state)
	}

	#[instrument(skip(self))]
	fn section(&mut self, strip: usize, start: usize, len: usize, reversed: bool) -> Section {
		let strip = &mut self.state[strip];
		let section = &mut strip[start..start + len];

		Section::new(section, reversed)
	}
}

///
/// strip 1: 0-148  149-308  309-391  392-474
/// strip 2: 0-148  149-308  309-350  351-436
/// strip 3: 0-106  107-153  154-208  209-308  309-350  351-442  443-474
///
pub struct Views<'a> {
	pub sections: [Section<'a>; 15],
}

impl<'a> Views<'a> {
	pub fn new(leds: &'a mut [[Rgba; LEDS_PER_STRIP]; STRIPS]) -> Self {
		let [first, second, third, ..] = leds;

		let (section1, rest) = first.split_at_mut(149);
		let (section2, rest) = rest.split_at_mut(309 - 149);
		let (section3, rest) = rest.split_at_mut(392 - 309);
		let (section4, _) = rest.split_at_mut(475 - 392);

		let (section5, rest) = second.split_at_mut(149);
		let (section6, rest) = rest.split_at_mut(309 - 149);
		let (section7, rest) = rest.split_at_mut(351 - 309);
		let (section8, _) = rest.split_at_mut(437 - 351);

		let (section9, rest) = third.split_at_mut(107);
		let (section10, rest) = rest.split_at_mut(153 - 107);
		let (section11, rest) = rest.split_at_mut(208 - 153);
		let (section12, rest) = rest.split_at_mut(308 - 208);
		let (section13, rest) = rest.split_at_mut(350 - 308);
		let (section14, rest) = rest.split_at_mut(442 - 350);
		let (section15, _) = rest.split_at_mut(475 - 442);

		let sections = [
			Section::new(section1, true),
			Section::new(section2, true),
			Section::new(section3, true),
			Section::new(section4, true),
			Section::new(section5, true),
			Section::new(section6, true),
			Section::new(section7, true),
			Section::new(section8, true),
			Section::new(section9, true),
			Section::new(section10, false),
			Section::new(section11, false),
			Section::new(section12, false),
			Section::new(section13, false),
			Section::new(section14, false),
			Section::new(section15, false),
		];

		Views { sections }
	}

	pub fn len(&self) -> usize {
		self.sections.len()
	}

	pub fn is_empty(&self) -> bool {
		self.sections.is_empty()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, Section<'a>> {
		self.sections.iter_mut()
	}

	pub fn iter(&mut self) -> Iter<'_, Section<'a>> {
		self.sections.iter()
	}
}

impl<'a> Index<usize> for Views<'a> {
	type Output = Section<'a>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.sections[index]
	}
}

impl<'a> IndexMut<usize> for Views<'a> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.sections[index]
	}
}

pub struct Section<'a> {
	slice:    &'a mut [Rgba],
	inverted: bool,
}

impl<'a> Section<'a> {
	pub fn new(slice: &'a mut [Rgba], inverted: bool) -> Self {
		Section { slice, inverted }
	}

	pub fn len(&self) -> usize {
		self.slice.len()
	}

	pub fn is_empty(&self) -> bool {
		self.slice.is_empty()
	}

	pub fn iter(&mut self) -> Box<dyn Iterator<Item = &'_ Rgba> + '_> {
		let iter = self.slice.iter();
		if self.inverted {
			Box::new(iter.rev())
		} else {
			Box::new(iter)
		}
	}

	pub fn iter_mut(&mut self) -> Box<dyn Iterator<Item = &'_ mut Rgba> + '_> {
		let iter = self.slice.iter_mut();
		if self.inverted {
			Box::new(iter.rev())
		} else {
			Box::new(iter)
		}
	}

	pub fn range<T: RangeBounds<usize> + Debug>(&mut self, range: T) -> Section<'_> {
		let start_bound = bound_to_num(range.start_bound(), true, 0, self.slice.len() - 1);
		let end_bound = bound_to_num(range.end_bound(), false, 0, self.slice.len() - 1);

		let max_idx = self.len();

		let range = if self.inverted {
			let start = max_idx - end_bound;
			let end = max_idx - start_bound;
			start..end
		} else {
			start_bound..end_bound
		};

		let slice = self.slice.index_mut(range);
		Section::new(slice, self.inverted)
	}

	pub fn set_aa_range<T: RangeBounds<f32>>(&mut self, range: T, val: &Rgba) {
		let start_bound = bound_to_num(
			range.start_bound(),
			true,
			0.0,
			(self.slice.len() - 1) as f32,
		);
		let end_bound = bound_to_num(range.end_bound(), false, 0.0, (self.slice.len() - 1) as f32);

		self.set_aa(start_bound, val);
		self.set_aa(end_bound, val);

		self.range((start_bound.ceil() as usize)..(end_bound.floor() as usize))
			.slice
			.fill_with(|| *val);
	}

	pub fn set_aa(&mut self, index: f32, val: &Rgba) {
		let lower = index.floor().max(0.0).min((self.len() - 1) as f32) as usize;
		let upper = index.ceil().max(0.0).min((self.len() - 1) as f32) as usize;

		if lower == upper {
			self[lower] = *val;
			return;
		}

		// let lower_influence = index - lower as f32;
		// let upper_influence = upper as f32 - index;

		let lower_influence = upper as f32 - index;
		let upper_influence = index - lower as f32;

		// info!(
		// 	"aa from {} to [{} ({}) .. {} ({})]",
		// 	index, lower, lower_influence, upper, upper_influence
		// );
		//
		// info!(
		// 	"lerp lower: lerp_color({:?}, {:?}, {}) = {:?}",
		// 	self[lower],
		// 	val,
		// 	lower_influence,
		// 	lerp_color(self[lower], val, lower_influence)
		// );

		self[lower] = self[lower].mix(val.into_linear(), lower_influence).into();
		self[upper] = self[upper].mix(val.into_linear(), upper_influence).into();
		// self[lower] = lerp_color(self[lower], val, lower_influence);
		// self[upper] = lerp_color(self[upper], val, upper_influence);
	}
}

// fn lerp_color(from: [u8; 3], to: [u8; 3], factor: f32) -> [u8; 3] {
// 	[
// 		lerp(from[0] as _, to[0] as _, factor) as _,
// 		lerp(from[1] as _, to[1] as _, factor) as _,
// 		lerp(from[2] as _, to[2] as _, factor) as _,
// 	]
// }

fn bound_to_num<T: Copy + std::ops::Add<Output = T> + From<u8>>(
	bound: Bound<&T>,
	start: bool,
	min: T,
	max: T,
) -> T {
	match bound {
		Bound::Included(n) => {
			if start {
				*n
			} else {
				*n + T::from(1)
			}
		}
		Bound::Excluded(n) => {
			if start {
				*n + T::from(1)
			} else {
				*n
			}
		}
		Bound::Unbounded => {
			if start {
				min
			} else {
				max
			}
		}
	}
}

impl<'a> Index<usize> for Section<'a> {
	type Output = Rgba;

	fn index(&self, mut index: usize) -> &Self::Output {
		assert!(index < self.slice.len());
		if self.inverted {
			index = self.slice.len() - 1 - index;
		}
		self.slice.index(index)
	}
}

impl<'a> IndexMut<usize> for Section<'a> {
	fn index_mut(&mut self, mut index: usize) -> &mut Self::Output {
		if self.inverted {
			index = self.slice.len() - 1 - index;
		}
		self.slice.index_mut(index)
	}
}
