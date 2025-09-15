import dotenv from 'dotenv';
import mkcert from 'vite-plugin-mkcert';
import { paraglideVitePlugin } from '@inlang/paraglide-js';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

dotenv.config({ path: '../.env' });
const { BACKEND_HOST } = process.env;
const API_BASE_URL = `${BACKEND_HOST ?? 'localhost'}:8080`;
console.log('API base URL: ', API_BASE_URL);

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit(),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide',
			strategy: ['url', 'cookie', 'baseLocale']
		}),
		mkcert()
	],
	server: {
		host: '0.0.0.0',
		https: {},
		proxy: {
			'/api': {
				target: `http://${API_BASE_URL}`,
				changeOrigin: true,
				secure: false
			},
			'/ws': {
				target: `ws://${API_BASE_URL}`,
				changeOrigin: true,
				secure: false
			}
		}
	}
});
