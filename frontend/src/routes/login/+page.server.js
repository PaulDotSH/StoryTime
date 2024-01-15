import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	if (locals.user) throw redirect(307, '/');
}

/** @type {import('./$types').Actions} */
