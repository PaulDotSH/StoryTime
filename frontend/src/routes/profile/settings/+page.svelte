<script>
    import { enhance } from '$app/forms';
    import ListErrors from '$lib/ListErrors.svelte';
    import * as api from '$lib/api.js'; // Import the API module

    /** @type {import('./$types').PageData} */
    export let data;

    /** @type {import('./$types').ActionData} */
    export let form;

    async function handleConfirmEmail(event) {
        event.preventDefault();

        try {
            const response = await api.post('resend'); // Use Axios instance to make the request
            if (response.data.errors) {
                console.error('Error sending confirmation:', response.data.errors);
                form.errors = response.data.errors;
            } else {
                console.log('Confirmation code sent successfully');
            }
        } catch (error) {
            console.error('Error:', error);
        }
    }

</script>

<svelte:head>
	<title>Settings • StoryTime</title>
</svelte:head>

<div class="settings-page">
	<div class="container page">
		<div class="row">
			<div class="col-md-6 offset-md-3 col-xs-12">
				<h1 class="text-xs-center">Your Settings</h1>

				<ListErrors errors={form?.errors} />

				<form
					use:enhance={() => {
						return ({ update }) => {
							// don't clear the form when we update the profile
							update({ reset: false });
						};
					}}
					method="POST"
					action="?/save"
				>
					<fieldset>
						<fieldset class="form-group">
							<input
								class="form-control form-control-lg"
								name="username"
								type="text"
								placeholder="Username"
								value={data.user.username}
							/>
						</fieldset>

						<fieldset class="form-group">
							<input
								class="form-control form-control-lg"
								name="email"
								type="email"
								placeholder="Email"
								value={data.user.email}
							/>
						</fieldset>

						<fieldset class="form-group">
							<textarea
								class="form-control form-control-lg"
								name="bio"
								rows="8"
								placeholder="Tell us a little about yourself!"
								value={data.user.bio}
							/>
						</fieldset>

						<fieldset class="form-group">
							<input
								class="form-control"
								name="image"
								type="text"
								placeholder="URL of profile picture"
								value={data.user.image}
							/>
						</fieldset>

						<fieldset class="form-group">
							<input
								class="form-control form-control-lg"
								name="password"
								type="password"
								placeholder="Change Password"
							/>
						</fieldset>

						<button class="btn btn-lg btn-primary pull-xs-right">Update Profile</button>
					</fieldset>
				</form>

				<hr />

				<form on:submit={handleConfirmEmail} use:enhance method="POST" action="?/confirmation">
					<button class="btn">Confirm your email</button>
				</form>

				<hr />

				<form use:enhance method="POST" action="?/logout">
					<button class="btn btn-outline-danger">Logout</button>
				</form>
			</div>
		</div>
	</div>
</div>
