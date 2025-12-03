import { getFears, isLoggedIn } from '$lib/server/requests.ts';
import type { Fear } from '$lib/types.ts';
import type { LayoutServerLoad } from './$types.js';

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
	const sessionId = cookies.get('session_id');
	const data = await getFears({ fetch, sessionId: sessionId! });
	return {
		isLoggedIn: await isLoggedIn({ fetch, sessionId: sessionId! }),
		data: data.message.VecData as Fear[]
	};
};
