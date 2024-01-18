/** @type {import('@sveltejs/kit').Handle} */
export function handle({ event, resolve }) {
    // Retrieve the Rust-generated token from the cookie
    const token = event.cookies.get('TOKEN');
    event.locals.user = token; // Store the token in locals

    return resolve(event);
}
