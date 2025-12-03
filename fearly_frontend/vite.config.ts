import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';
import { loadConfig } from './src/lib/server/+config.ts';

const config = loadConfig();
const fae = config.routers.api[0];
const apiUrl = `${fae.protocol}://${fae.hostname}:${fae.port}`;
const DEBUG = config.debug;

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), SvelteKitPWA()],
	server: {
		allowedHosts: true,
		proxy: {
			'/server/login': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/login/, '/login')
			},
			'/server/logout': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/logout/, '/logout')
			},
			'/server/logged_in': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/logged_in/, '/logged_in')
			},
			'/server/logged_in_bool': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/logged_in_bool/, '/logged_in_bool')
			},
			'/server/add_fear': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/add_fear/, '/add_fear')
			},
			'/server/edit_fear': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/edit_fear/, '/edit_fear')
			},

			'/server/get_fears': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/get_fears/, '/get_fears')
			},

			'/server/delete_fear': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG,
				rewrite: (path) => path.replace(/^\/server\/delete_fear\/([^/]+)/, '/delete_fear/$1')
			},
			'/media': {
				target: apiUrl,
				changeOrigin: true,
				secure: !DEBUG
			}
		}
	}
});
