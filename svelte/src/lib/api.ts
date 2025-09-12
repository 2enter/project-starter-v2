import axios from 'axios';

import { Api } from '@2enter/web-kit/runtime';

const api = new Api(axios.create());

export async function getServerHealth() {
	return api.fetch<boolean>({ url: '/api/health', method: 'GET' });
}
