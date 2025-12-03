import type { Fear, GenericResponse, VecData } from '$lib/types.ts';
import { loadConfig } from './+config.ts';

// fae = first api endpoint
const config = loadConfig();
const fae = config.routers.api[0];
const apiUrl = `${fae.protocol}://${fae.hostname}:${fae.port}`;

export const isLoggedIn = async ({
	fetch,
	sessionId
}: {
	fetch: typeof globalThis.fetch;
	sessionId: string;
}) => {
	const response = await fetch(`${apiUrl}/logged_in_bool`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		}
	});

	if (!response.ok) {
		throw new Error('Failed to check if logged in');
	}

	return (await response.json()) as boolean;
};

export const getFears = async ({
	fetch,
	sessionId
}: {
	fetch: typeof globalThis.fetch;
	sessionId: string;
}) => {
	const response = await fetch(`${apiUrl}/get_fears`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json',
			Cookie: `session_id=${sessionId}`
		}
	});
	if (response.status === 401) {
		return (await response.json()) as GenericResponse<VecData<Fear>>;
	}
	if (!response.ok) {
		throw new Error('Failed to fetch items');
	}

	return (await response.json()) as GenericResponse<VecData<Fear>>;
};
