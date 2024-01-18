<script>
    import { enhance } from '$app/forms';
    import ListErrors from '$lib/ListErrors.svelte';

	
	export let username;
    export let email;

    // /** @type {import('./$types').PageData} */
    // export let data;

    /** @type {import('./$types').ActionData} */
    export let form;

	console.log({ username, email });
</script>

<svelte:head>
	<title>Settings • StoryTime</title>
</svelte:head>

<div class="user-info">
    <p>Username: {username}</p>
</div>

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
								value={username}
							/>
						</fieldset>

						<fieldset class="form-group">
							<input
								class="form-control form-control-lg"
								name="email"
								type="email"
								placeholder="Email"
								value={email}
							/>
						</fieldset>

						<fieldset class="form-group">
							<textarea
								class="form-control form-control-lg"
								name="bio"
								rows="8"
								placeholder="Tell us a little about yourself!"
								value={""}
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

				<form use:enhance method="POST" action="?/resend">
					<button class="btn">Send confirmation mail</button>
				</form>


				<hr />

				<form use:enhance method="POST" action="?/confirmation">
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
