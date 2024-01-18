import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

export async function load({ request, cookies, locals }) {
	if (!locals.user) throw redirect(302, '/login');

    try {

		let cookieData = `TOKEN=${cookies.get('TOKEN')}`;
	
		const response = await api.get('profile', {} , cookieData);
		console.log({ username: response.data.username, email: response.data.email });


        return {
            props: {
                username: response.data.username,
                email: response.data.email
            }
        };
    } catch (error) {
        console.error('Error fetching profile data:', error);
        return {
            props: {
                error: 'Failed to fetch profile data.'
            }
        };
    }

}

/** @type {import('./$types').Actions} */
export const actions = {
	logout: async ({ cookies, locals }) => {
		cookies.delete('TOKEN', { path: '/' });
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
        cookies.set('TOKEN', token, { 
            path: '/', 
            maxAge: 604800
        });

		locals.user = body.user;
	},

    confirmation: async () => {
        throw redirect(307, '/confirmation'); 
    },

	resend: async () => {
        throw redirect(307, '/resend'); 
    },
	
};