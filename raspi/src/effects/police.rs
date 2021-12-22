use educe::Educe;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{config::color::Color, controller::Controller, db, effects::prelude::*};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema, Educe)]
#[educe(Default)]
pub struct PoliceConfig {
	color: Color,
}

pub struct Police {
	config: PoliceConfig,
	db:     sled::Tree,
}

impl Police {
	pub fn new(mut db: sled::Tree) -> Self {
		let mut effect = Police {
			config: db::load_config(&mut db),
			db,
		};

		effect.set_config(effect.config);

		effect
	}

	fn set_config(&mut self, config: PoliceConfig) {
		self.config = config;
	}

	fn run(&mut self, ctrl: &mut impl LedController) {
		let color = self.config.color.value().into();
		set_all_delay(ctrl, &color, true, 150);
		set_all_delay(ctrl, &color, false, 47);
		set_all_delay(ctrl, &color, true, 16);
		set_all_delay(ctrl, &color, false, 24);
		set_all_delay(ctrl, &color, true, 16);
		set_all_delay(ctrl, &color, false, 24);
		set_all_delay(ctrl, &color, true, 16);
		set_all_delay(ctrl, &color, false, 186);
	}
}

effect!(Police, PoliceConfig);
