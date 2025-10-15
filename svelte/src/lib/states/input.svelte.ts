import { Locale, type Interaction } from '@/types/shared';
import { getContext, setContext } from 'svelte';

class InputState {
	result = $state<Interaction>();

	readonly submittable = $derived(true);

	insertTestResultData = () => {
		this.result = {
			id: '86add349-c32d-48c2-bcd5-9b016044349f',
			createdAt: new Date(),
			locale: Locale.En,
			duration: 10000
			// add whatever you want
		};

		return this;
	};

	reset = () => {
		this.result = undefined;
	};
}

const INPUT_STATE_CTX = 'INPUT_STATE';

function setInputState() {
	return setContext(INPUT_STATE_CTX, new InputState());
}

function getInputState() {
	return getContext<ReturnType<typeof setInputState>>(INPUT_STATE_CTX);
}

export { setInputState, getInputState };
