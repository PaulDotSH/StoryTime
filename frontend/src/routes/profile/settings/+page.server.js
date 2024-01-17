import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

export function load({ locals }) {
	if (!locals.user) throw redirect(302, '/login');
}

/** @type {import('./$types').Actions} */
export const actions = {
	logout: async ({ cookies, locals }) => {
		cookies.delete('jwt', { path: '/' });
		locals.user = null;
	},



	save: async ({ cookies, locals, request }) => {
		if (!locals.user) throw error(401);

		const data = await request.formData();

		const user = {
			username: data.get('username'),
			email: data.get('email'),
			password: data.get('password'),
			image: data.get('image'),
			bio: data.get('bio')
		};

		const body = await api.put('user', { user }, locals.user.token);
		if (body.errors) return fail(400, body.errors);

		console.log(body.data);
        const token = body.data;
        cookies.set('jwt', token, { 
            path: '/', 
            maxAge: 604800
        });

		locals.user = body.user;
	},

    confirmation: async () => {
		const body = await api.post('resend');
		if (body.errors) return fail(500, body.errors);
		console.log(body.data);
        throw redirect(307, '/confirmation'); 
    },
};
