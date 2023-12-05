import { redirect, fail } from '@sveltejs/kit';
import * as api from '$lib/api.js';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	if (!locals.user) throw redirect(302, `/login`);
}

/** @type {import('./$types').Actions} */
export const actions = {

};
