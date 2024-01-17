import axios from 'axios';
import { error } from '@sveltejs/kit';

const base = 'http://127.0.0.1:5431';

async function send({ method, path, data, token }) {
    const headers = {
        'Content-Type': 'application/json'
    };


    // Adding withCredentials: true to include cookies in the request
    const config = {
        method: method,
        url: `${base}/${path}`,
        data: data,
        headers: headers,
        withCredentials: true
        //set credentials to same-origin to include cookies in the request
        //credentials: 'same-origin'
    };

    try {
        const response = await axios(config);

        if (response.status === 200) {
            return response;
        }

        throw error(response.status, response.statusText);
    } catch (err) {
        // Improved error handling
        throw error(err.response?.status || 500, err.response?.statusText || 'Server error');
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
