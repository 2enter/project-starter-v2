<script lang="ts">
	import { getSysState } from '@/states';
	const sysState = getSysState();
</script>

<button class="btn btn-primary" onclick={sysState.timer.start}>Start!</button>
