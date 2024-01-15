import axios from 'axios';
import { error } from '@sveltejs/kit';

const base = 'http://127.0.0.1:5431'; // Ensure you include the protocol (http/https)

async function send({ method, path, data, token }) {
    const headers = {
		'Content-Type': 'application/json'
	};

    try {
        const response = await axios[method.toLowerCase()](`${base}/${path}`, data, { headers });

        if (response.status === 200) {
            return response;
        }

        throw error();
    } catch (error) {
        throw error;
    }
}

export function get(path, token) {
    return send({ method: 'GET', path, token });
}

export function del(path, token) {
    return send({ method: 'DELETE', path, token });
}

export function post(path, data, token) {
    return send({ method: 'POST', path, data, token });
}

export function put(path, data, token) {
    return send({ method: 'PUT', path, data, token });
}
