<script lang="ts">
	import { onMount } from 'svelte';

	type Article = { id: number; title: string; url: string; source: string; published_at: string };
	type MuteWord = { id: number; word: string };

	let articles = $state<Article[]>([]);
	let muteWords = $state<MuteWord[]>([]);
	let showAll = $state(false);
	let newMuteWord = $state('');

	const API_BASE = 'http://127.0.0.1:3000/api';

	// バッジの色を決定する関数
	function getBadgeStyle(source: string) {
		if (source === 'Zenn') return 'background: #3EA8FF; color: white;';
		if (source === 'Qiita') return 'background: #55C500; color: white;';
		if (source === 'Hatena Blog') return 'background: #00A4DE; color: white;';
		if (source === 'ZDNet') return 'background: #CC0000; color: white;';
		// その他（未知のドメインなど）はグレー
		return 'background: #666666; color: white;';
	}

	async function fetchArticles() {
		const res = await fetch(`${API_BASE}/articles?show_all=${showAll}`);
		articles = await res.json();
	}

	async function fetchMuteWords() {
		const res = await fetch(`${API_BASE}/mutewords`);
		muteWords = await res.json();
	}

	async function markAsRead(id: number, url: string) {
		await fetch(`${API_BASE}/articles/${id}/read`, { method: 'POST' });
		articles = articles.filter((a) => a.id !== id);
		window.open(url, '_blank');
	}

	async function addMuteWord() {
		if (!newMuteWord.trim()) return;
		await fetch(`${API_BASE}/mutewords`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ word: newMuteWord })
		});
		newMuteWord = '';
		await fetchMuteWords();
		await fetchArticles();
	}

	onMount(() => {
		fetchArticles();
		fetchMuteWords();
	});
</script>

<main style="max-width: 800px; margin: 0 auto; padding: 1rem; font-family: sans-serif;">
	<h1>マイニュース</h1>

	<section style="margin-bottom: 2rem; padding: 1rem; background: #f4f4f4; border-radius: 8px;">
		<h2>ミュートワード設定</h2>
		<div style="display: flex; gap: 0.5rem; margin-bottom: 1rem;">
			<input type="text" bind:value={newMuteWord} placeholder="ミュートする単語を入力" />
			<button onclick={addMuteWord}>追加</button>
		</div>
		<div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
			{#each muteWords as mw}
				<span style="background: #ddd; padding: 4px 8px; border-radius: 4px; font-size: 0.9rem;">
					{mw.word}
				</span>
			{/each}
		</div>
	</section>

	<section>
		<div
			style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;"
		>
			<h2>未読記事 ({articles.length}件)</h2>
			<button
				onclick={() => {
					showAll = !showAll;
					fetchArticles();
				}}
			>
				{showAll ? '30日以内の未読のみ表示' : '30日以前の未読も表示'}
			</button>
		</div>

		<ul style="list-style: none; padding: 0;">
			{#each articles as article}
				<li style="margin-bottom: 1rem; border-bottom: 1px solid #eee; padding-bottom: 0.5rem;">
					<div style="font-size: 0.8rem; color: #666; margin-bottom: 0.4rem;">
						<span
							style="{getBadgeStyle(
								article.source
							)} padding: 2px 6px; border-radius: 4px; margin-right: 8px; font-weight: bold;"
						>
							{article.source}
						</span>
						{new Date(article.published_at).toLocaleString()}
					</div>
					<a
						href={article.url}
						onclick={(e) => {
							e.preventDefault();
							markAsRead(article.id, article.url);
						}}
						style="font-size: 1.1rem; color: #0056b3; text-decoration: none; font-weight: bold;"
					>
						{article.title}
					</a>
				</li>
			{/each}
		</ul>
	</section>
</main>
