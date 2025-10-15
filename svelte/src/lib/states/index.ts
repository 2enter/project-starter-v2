import { getSysState } from './sys';
import { getInputState } from './input.svelte';

export * from './sys';
export * from './input.svelte';

export function getAllStates() {
	return {
		sysState: getSysState(),
		inputState: getInputState()
	};
}
