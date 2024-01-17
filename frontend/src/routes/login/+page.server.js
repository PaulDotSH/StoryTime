import { fail, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js'; // Update this to point to your Rust backend

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
    if (locals.user) throw redirect(307, '/');
}

/** @type {import('./$types').Actions} */
export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();

        // Update the API call to interact with your Rust backend
        const response = await api.post('login', {
            username: data.get('username'),
            password: data.get('password')
        });

        if (response.status !== 200) {
            return fail(response.status, { errors: 'Login failed' });
        }
		console.log(response.data);
        const token = response.data;
        cookies.set('TOKEN', token, { 
            path: '/', 
            maxAge: 604800 // 1 week
        });

        throw redirect(307, '/');
    }
};
