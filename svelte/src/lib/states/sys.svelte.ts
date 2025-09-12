import { getContext, setContext } from 'svelte';
import { MAX_PAGE_NUM } from '@/pages';

class SysState {
	processing = $state(false);
	errorMessage = $state<string | null>(null);
	pageNum = $state<number>(0);
	dialog = $state<HTMLDialogElement>();

	popError = (message: string) => {
		this.errorMessage = message;
		if (!this.dialog) return;
		this.dialog.showModal();
	};

	startProcess = () => {
		this.processing = true;
	};

	endProcess = () => {
		this.processing = false;
	};

	closeError = () => {
		this.errorMessage = null;
		if (!this.dialog) return;
		this.dialog.close();
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
