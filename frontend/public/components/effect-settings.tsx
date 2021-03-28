import { JSONSchema7 } from "json-schema";
import { useMemo } from "preact/hooks";
import clsx from "clsx";
import { styled } from "goober";
import { Sliders, Zap } from "preact-feather";

import { EffectData } from "../state/api";
import { STATES } from "../state/state";

import styles from "./effect-settings.module.css";
import { prettyName } from "../util/pretty-names";

export function EffectSettings({
	effectData,
	setEffectConfig,
	state,
}: {
	effectData: EffectData;
	setEffectConfig: (config: Record<string, any>) => void;
	state: string;
}) {
	if (state === STATES.LOADING) {
		return <p>loading...</p>;
	}

	if (state === STATES.ERROR) {
		return <p>something went wrong</p>;
	}

	function patchAndUpdateConfig(field, value) {
		let config: { [name: string]: any } = JSON.parse(JSON.stringify(effectData.config));
		config[field] = value;
		setEffectConfig(config);
	}

	return (
		<>
			<h2 class={styles.title}>
				<Sliders /> &nbsp; {prettyName(effectData.name)}
			</h2>
			<Settings
				config={effectData.config}
				schema={effectData.schema}
				update={patchAndUpdateConfig}
			/>
		</>
	);
}

type Field = {
	name: string;
	value: any;
	schema: JSONSchema7 | null;
};

function Settings({
	config,
	schema,
	update,
}: {
	config: Record<string, any>;
	schema: JSONSchema7;
	update: (field: string, value: any) => void;
}) {
	let fields: Field[] = useMemo(() => {
		if (schema.type !== "object") {
			return [];
		}

		return Object.keys(config).map((name) => {
			let propertySchema = schema.properties[name];
			if (typeof propertySchema === "boolean") {
				propertySchema = null;
			}

			return {
				name,
				value: config[name],
				schema: propertySchema as JSONSchema7 | null,
			};
		});
	}, [config, schema]);

	if (fields.length === 0) {
		return <p>No config options for this effect.</p>;
	}

	return (
		<form onSubmit={(e) => e.preventDefault} class={styles.form}>
			{fields.map((f) => (
				<Setting field={f} onChange={(value) => update(f.name, value)} />
			))}
		</form>
	);
}

function getInputType(schema: JSONSchema7): HTMLInputElement["type"] {
	switch (schema.type) {
		case "number":
		case "integer":
			return "number";

		case "boolean":
			return "checkbox";

		default:
			return "text";
	}
}

function getValue(schema: JSONSchema7, el: HTMLInputElement) {
	switch (schema.type) {
		case "number":
		case "integer":
			return el.valueAsNumber;

		case "boolean":
			return el.checked;

		default:
			return el.value;
	}
}

function readableValue(schema: JSONSchema7, value: any) {
	switch (schema.type) {
		case "number":
		case "integer":
			return Math.round(value * 1000) / 1000;

		default:
			return value;
	}
}

function Setting({ field, onChange }: { field: Field; onChange: (value: any) => void }) {
	let id = `input__${field.name}`;

	let inputType = getInputType(field.schema);

	let label = prettyName(field.name);
	if (field.schema === null) {
		label = label + " (invalid schema)";
	}

	let value = readableValue(field.schema, field.value);

	return (
		<fieldset class={clsx({ error: field.schema === null })}>
			<label htmlFor={id}>{label}</label>
			<input
				type={inputType}
				disabled={field.schema === null}
				value={value}
				onChange={(e) => onChange(getValue(field.schema, e.currentTarget))}
			/>
		</fieldset>
	);
}
