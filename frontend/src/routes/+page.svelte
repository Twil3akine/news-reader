<script>
	import { onMount } from 'svelte';

	let articles = [];
	let loading = true;

	async function load() {
		loading = true;
		const res = await fetch('http://127.0.0.1:3000/articles');
		articles = await res.json();
		loading = false;
	}

	onMount(load);
</script>

<main>
	<h1>News Reader</h1>

	{#if loading}
		<p>Loading...</p>
	{:else}
		{#each articles as article}
			<div class="card {article.is_read ? 'read' : ''}">
				<a href={article.url} target="_blank">
					<h2>{article.title}</h2>
				</a>
				<p class="date">{article.published_at}</p>
			</div>
		{/each}
	{/if}
</main>

<style>
	main {
		max-width: 900px;
		margin: 40px auto;
		font-family: -apple-system, BlinkMacSystemFont, sans-serif;
	}

	h1 {
		text-align: center;
		margin-bottom: 40px;
	}

	.card {
		background: white;
		padding: 20px;
		margin-bottom: 16px;
		border-radius: 16px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		transition: 0.2s;
	}

	.card:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 18px rgba(0, 0, 0, 0.12);
	}

	.card.read {
		opacity: 0.5;
	}

	h2 {
		margin: 0 0 8px 0;
		font-size: 18px;
	}

	.date {
		font-size: 13px;
		color: gray;
	}
</style>
