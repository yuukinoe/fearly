import type { Handle } from '@sveltejs/kit';
import { loadConfig } from '$lib/server/+config.ts';

const config = loadConfig();
// fae = first api endpoint
const fae = config.routers.api[0];
const apiUrl = `${fae.protocol}://${fae.hostname}:${fae.port}`;
// const DEBUG = config.debug;

interface FetchOptions extends RequestInit {
	duplex?: 'half';
}

interface ProxyPatterns {
	path: string;
	backend: string;
}

const proxyPatterns: ProxyPatterns[] = [
	{ path: '/server/login', backend: '/login' },
	{ path: '/server/logout', backend: '/logout' },
	{ path: '/server/logged_in', backend: '/logged_in' },
	{ path: '/server/logged_in_bool', backend: '/logged_in_bool' },
	{ path: '/server/add_fear', backend: '/add_fear' },
	{ path: '/server/edit_fear', backend: '/edit_fear' },
	{ path: '/server/get_fears', backend: '/get_fears' },

	{ path: '/server/delete_fear/:item_id', backend: '/delete_fear/:item_id' }
];

function compileRoutePattern(path: string) {
	const paramNames: string[] = [];
	const regex = new RegExp(
		'^' +
			path.replace(/:[^/]+/g, (match) => {
				paramNames.push(match.substring(1));
				return '([^/]+)';
			}) +
			'$'
	);
	return { regex, paramNames };
}

export const handle: Handle = async ({ event, resolve }) => {
	const url = event.url;
	const pathname = url.pathname;

	for (const route of proxyPatterns) {
		const { regex, paramNames } = compileRoutePattern(route.path);
		const match = pathname.match(regex);

		if (match) {
			const params: Record<string, string> = {};
			paramNames.forEach((name, i) => (params[name] = match[i + 1]));

			let targetPath = route.backend;
			for (const [key, value] of Object.entries(params)) {
				targetPath = targetPath.replace(`:${key}`, value);
			}

			const targetUrl = `${apiUrl}${targetPath}${url.search}`;

			try {
				const headers = new Headers(event.request.headers);
				headers.delete('content-length');
				if (!headers.has('Origin')) headers.set('Origin', config.routers.domain);

				const fetchOptions: FetchOptions = {
					method: event.request.method,
					headers
				};

				if (
					event.request.body &&
					event.request.method !== 'GET' &&
					event.request.method !== 'HEAD'
				) {
					fetchOptions.body = event.request.body;
					fetchOptions.duplex = 'half';
				}

				const response = await fetch(targetUrl, fetchOptions);

				return new Response(response.body, {
					status: response.status,
					statusText: response.statusText,
					headers: response.headers
				});
			} catch (error) {
				console.error(`Proxy error for ${pathname}:`, error);
				return new Response('Proxy Error', { status: 500 });
			}
		}
	}

	if (pathname.startsWith('/media/')) {
		const targetUrl = `${apiUrl}${pathname}${url.search}`;
		try {
			const headers = new Headers(event.request.headers);
			headers.delete('content-length');
			if (!headers.has('Origin')) headers.set('Origin', config.routers.domain);

			const fetchOptions: FetchOptions = {
				method: event.request.method,
				headers
			};

			if (event.request.body && event.request.method !== 'GET' && event.request.method !== 'HEAD') {
				fetchOptions.body = event.request.body;
				fetchOptions.duplex = 'half';
			}

			const response = await fetch(targetUrl, fetchOptions);

			return new Response(response.body, {
				status: response.status,
				statusText: response.statusText,
				headers: response.headers
			});
		} catch (error) {
			console.error('Media proxy error:', error);
			return new Response('Media Proxy Error', { status: 500 });
		}
	}

	return resolve(event);
};
