import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

export function load({ locals }) {
	if (!locals.user) throw redirect(302, '/login');
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ cookies, request }) => {
		let cookieData = `TOKEN=${cookies.get('TOKEN')}`;
	
		const body = await api.post('api/resend', {}, cookieData);

		if (body.errors) {
			return fail(401, body);
		}
		throw redirect(307, '/');
	}
};
