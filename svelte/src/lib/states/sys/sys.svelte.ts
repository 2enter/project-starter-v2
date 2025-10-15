import { getContext, setContext } from 'svelte';
import { MAX_PAGE_NUM } from '@/pages';
import { Timer } from './timer.svelte';
import { Dialog } from './dialog.svelte';

class SysState {
	processing = $state(false);
	pageNum = $state<number>(0);
	dialog = new Dialog();
	timer = new Timer();

	startProcess = () => {
		this.processing = true;
	};

	endProcess = () => {
		this.processing = false;
	};

	navigate = (step?: any) => {
		if (typeof step !== 'number') {
			step = 1;
		}

		let result = this.pageNum + step;

		if (result < 0 || result > MAX_PAGE_NUM) {
			console.error('invalid page navigation');
			return;
		}

		this.routeTo(result);
	};

	routeTo = (num: number) => {
		if (num === this.pageNum) return;
		setTimeout(() => (this.pageNum = num));
	};
}

const SYS_STATE_CTX = 'SYS_STATE';

function setSysState() {
	return setContext(SYS_STATE_CTX, new SysState());
}

function getSysState() {
	return getContext<ReturnType<typeof setSysState>>(SYS_STATE_CTX);
}

export { setSysState, getSysState };
