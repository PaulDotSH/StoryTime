<script>
	import { enhance } from '$app/forms';
	import ListErrors from '$lib/ListErrors.svelte';

	/** @type {import('./$types').ActionData} */
	export let form;
</script>

<svelte:head>
	<title>Confirmation • StoryTime</title>
</svelte:head>

<div class="auth-page">
	<div class="container page">
		<div class="row">
			<div class="col-md-6 offset-md-3 col-xs-12">
				<h1 class="text-xs-center">Confirm account</h1>
				<br>

				<ListErrors errors={form?.errors} />

				<form use:enhance method="POST">
					<button class="btn btn-lg btn-primary pull-xs-right">Resend</button>
				</form>
			</div>
		</div>
	</div>
</div>
