import axios from 'axios';
import { fail, redirect, error } from '@sveltejs/kit';

const base = 'http://127.0.0.1:5431';

async function send({ method, path, data, cookies }) {
    const headers = {
        'Content-Type': 'application/json'
    };

    

    if (cookies) {
        headers.Cookie = cookies;
        
    }

    const config = {
        method: method,
        url: `${base}/${path}`,
        data: data,
        headers: headers,
        withCredentials: true,
        credentials: 'same-origin'
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

export function get(path, data, cookies) {
    return send({ method: 'GET', path , cookies});
}

export function del(path, data,  cookies) {
    return send({ method: 'DELETE', path });
}

export function post(path, data, cookies) {
    return send({ method: 'POST', path, data,  cookies });
}

export function put(path, data, cookies) {
    return send({ method: 'PUT', path, data });
}
