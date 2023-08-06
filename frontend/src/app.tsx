import { LocationProvider } from "preact-iso";

import { useMachine } from "./util/state-machine";
import { actions, machine, MESSAGES, STATES } from "./state/state";
import { EffectSettings } from "./components/effect-settings";
import { Sidebar } from "./components/sidebar";

import styles from "./app.module.css";
import { useCallback } from "preact/hooks";
import { DisplayState, EffectConfig } from "./state/api.ts";

export function App() {
	const [
		{
			value: state,
			context: { config, segments, effects, presets, state: displayState },
		},
		send,
	] = useMachine(machine, actions);

	// SET_CONFIG = "setConfig",
	// 	SET_EFFECT_CONFIG = "setEffectConfig",
	// 	SET_SEGMENTS = "setSegments",
	// 	SET_PRESET = "setPreset",
	// 	LOAD_PRESET = "loadPreset",
	// 	SAVE_PRESET = "savePreset",
	// 	SET_STATE = "setState",

	// let setConfig = useCallback(
	// 	(config: Config) => {
	// 		send({
	// 			type: MESSAGES.SET_CONFIG,
	// 			config,
	// 		});
	// 	},
	// 	[send]
	// );
	//
	// let setEffectConfig = useCallback(
	// 	(idx: number, config: EffectConfig) => {
	// 		send({
	// 			type: MESSAGES.SET_EFFECT_CONFIG,
	// 			idx,
	// 			config,
	// 		});
	// 	},
	// 	[send]
	// );
	//
	// let setSegments = useCallback(
	// 	(segments: Segments) => {
	// 		send({
	// 			type: MESSAGES.SET_SEGMENTS,
	// 			segments,
	// 		});
	// 	},
	// 	[send]
	// );
	//
	// let setPreset = useCallback(
	// 	(name: string, state: DisplayState) => {
	// 		send({
	// 			type: MESSAGES.SET_PRESET,
	// 			name,
	// 			state,
	// 		});
	// 	},
	// 	[send]
	// );

	// let activeEffectData: EffectData | null = useMemo(() => {
	// 	if (!activeEffect) return null;
	// 	let data = effects[activeEffect];
	// 	return data == null ? null : data;
	// }, [activeEffect, effects]);

	let loadPreset = useCallback(
		(name: string) => {
			send({
				type: MESSAGES.LOAD_PRESET,
				name,
			});
		},
		[send]
	);

	let setEffectConfig = useCallback(
		(idx: number, effect: string, config: EffectConfig) => {
			send({
				type: MESSAGES.SET_EFFECT_CONFIG,
				idx,
				effect,
				config,
			});
		},
		[send]
	);

	return (
		<LocationProvider>
			<Sidebar
				state={state}
				displayState={displayState}
				effects={effects}
				presets={presets}
				loadPreset={loadPreset}
				setEffectConfig={setEffectConfig}
			/>
			<main class={styles.main}>
				{state === STATES.ERROR && (
					<div class="error">
						<p>Something went wrong</p>
						<button onClick={() => send({ type: MESSAGES.RETRY })}>Retry</button>
					</div>
				)}
				{state === STATES.LOADING && <p>loading...</p>}

				{displayState.effects[0] != null && (
					<EffectSettings
						effectState={displayState.effects[0]}
						state={state}
						effectData={effects[displayState.effects[0].effect]}
						setEffectConfig={setEffectConfig}
					/>
				)}
			</main>
		</LocationProvider>
	);
}
