<script>
	import { scale } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import { enhance } from '$app/forms';
	import ListErrors from '$lib/ListErrors.svelte';

	export let story;
	export let errors;
</script>

<div class="story-page">
	<div class="container page">
		<div class="row">
			<div class="col-md-10 offset-md-1 col-xs-12">
				<ListErrors {errors} />

				<form use:enhance method="POST">
					<fieldset class="form-group">
						<input
							name="title"
							class="form-control form-control-lg"
							placeholder="Story Name"
							value={story.title}
						/>
					</fieldset>

					<fieldset class="form-group">
						<input
							name="description"
							class="form-control"
							placeholder="Story Genre"
							value={story.description}
						/>
					</fieldset>

					<fieldset class="form-group">
						<textarea
							name="body"
							class="form-control"
							rows="8"
							placeholder="Tell us your story!"
							value={story.body}
						/>
					</fieldset>

					<fieldset class="form-group">
						<input
							class="form-control"
							placeholder="Add tags to your story"
							on:keydown={(event) => {
								if (event.key === 'Enter') {
									event.preventDefault();
									if (!story.tagList.includes(event.target.value)) {
										story.tagList = [...story.tagList, event.target.value];
									}

									event.target.value = '';
								}
							}}
						/>
					</fieldset>

					<div class="tag-list">
						{#each story.tagList as tag, i (tag)}
							<button
								transition:scale|local={{ duration: 200 }}
								animate:flip={{ duration: 200 }}
								class="tag-default tag-pill"
								on:click|preventDefault={() => {
									story.tagList = [
										...story.tagList.slice(0, i),
										...story.tagList.slice(i + 1)
									];
								}}
								aria-label="Remove {tag} tag"
							>
								<i class="ion-close-round" />
								{tag}
							</button>
						{/each}
					</div>

					{#each story.tagList as tag}
						<input hidden name="tag" value={tag} />
					{/each}

					<button class="btn btn-lg pull-xs-right btn-primary">Post Story</button>
				</form>
			</div>
		</div>
	</div>
</div>

<style>
	.tag-pill {
		border: none;
	}
</style>
