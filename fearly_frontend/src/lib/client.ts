// const apiUrl = '/server';

import type { AddFear, Fear } from './types.ts';

export const logoutUser = async () => {
	const response = await fetch(`/server/logout`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to logout user');
	}

	return (await response.json()) as boolean;
};

export const loginUser = async (user: { username: string; password: string }) => {
	// console.log('Logging in user:', user);
	const response = await fetch(`/server/login`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify({
			username: user.username,
			password: user.password
		})
	});
	if (!response.ok) return false;
	return true;
};

export const addFear = async (data: AddFear) => {
	const form = new URLSearchParams();
	form.append('title', data.title);
	form.append('content', data.content);
	form.append('reaction', data.reaction);
	form.append('emotion', data.emotion.toString());
	form.append('date', data.date);
	form.append('time', data.time);
	const response = await fetch(`/server/add_fear`, {
		method: 'POST',
		credentials: 'include',
		body: form.toString()
	});
	if (!response.ok) return false;
	return true;
};

export const deleteFear = async (id: string) => {
	const response = await fetch(`/server/delete_fear/${id}`, {
		method: 'DELETE',
		credentials: 'include'
	});
	if (!response.ok) return false;
	return true;
};

export const updateFear = async (data: Fear, date: string, time: string) => {
	const form = new URLSearchParams();

	form.append('id', data.id.id.String);
	form.append('title', data.title);
	form.append('content', data.content);
	form.append('reaction', data.reaction);
	form.append('emotion', data.emotion.toString());
	form.append('date', date);
	form.append('time', time);
	const response = await fetch(`/server/edit_fear`, {
		method: 'PATCH',
		credentials: 'include',
		body: form.toString()
	});
	if (!response.ok) return false;
	return true;
};
