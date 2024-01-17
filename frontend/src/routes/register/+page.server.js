import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	if (locals.user) throw redirect(307, '/');
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ cookies, request }) => {
		const data = await request.formData();
        
        const body = await api.post('register', {
            username: data.get('username'),
            password: data.get('password'),
            email:  data.get('email')
    });
        

        if (body.errors) {
            return fail(401, body);
        }

        console.log(body.data);
        const token = body.data;
        cookies.set('jwt', token, { 
            path: '/', 
            maxAge: 604800
        });

        throw redirect(307, '/');
    }
};
