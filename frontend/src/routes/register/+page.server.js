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

        const value = btoa(JSON.stringify(body.user));
        cookies.set('jwt', value, { path: '/' });

        // // Temporary this will be here because user endpoint isn't ready yet. When it's ready, this will be moved to /confirmation page
        // const body1 = await api.post('resend', { email });

        // if (body1.errors) {
        //     console.error('Error in the second request:', body1.errors);
        // }

        throw redirect(307, '/');
    }
};
