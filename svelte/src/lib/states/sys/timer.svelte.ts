import moment from 'moment';

export class Timer {
	startTime = $state<number>();

	start = () => {
		this.startTime = Date.now();
	};

	get duration() {
		if (!this.startTime) return 0;
		return Date.now() - this.startTime;
	}

	get seconds() {
		return Math.floor(this.duration / 1000);
	}

	formattedDuration = (format: string) => {
		return moment(this.duration).format(format);
	};
}
