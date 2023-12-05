<script>
	import Editor from './Editor.svelte';

	/** @type {import('./$types').ActionData} */
	export let form;
</script>

<Editor
	story={{
		title: '',
		description: '',
		body: '',
		tagList: []
	}}
	errors={form?.errors}
/>
