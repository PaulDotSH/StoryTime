import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

export function load({ locals }) {
	if (!locals.user) throw redirect(302, '/login');
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		const id = data.get('confirmationToken');
		const body = await api.post('confirm/' + id);

		if (body.errors) {
			return fail(401, body);
		}
		throw redirect(307, '/');
	}
};
