export class Dialog {
	dom?: HTMLDialogElement;
	message = $state<string>();
	header = $state<string>();
	closeBtnText = $state<string>();
	opened = $state(false);
	onclose = () => {};

	pop = (args: {
		message: string;
		header: string;
		closeBtnText?: string;
		onclose?: () => void;
	}) => {
		this.message = args.message;
		this.header = args.header;
		this.closeBtnText = args.closeBtnText ?? '關閉 Close';

		if (args.onclose) this.onclose = args.onclose;

		this.dom?.showModal();
		this.opened = true;
	};

	close = () => {
		if (!this.dom) return;
		this.dom.close();
		this.onclose();

		this.message = undefined;
		this.header = undefined;
		this.closeBtnText = undefined;
		this.onclose = () => {};

		this.opened = false;
	};
}
