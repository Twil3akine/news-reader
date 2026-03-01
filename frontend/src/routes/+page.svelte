<script lang="ts">
	import { onMount } from 'svelte';

	type Article = {
		id: number;
		title: string;
		url: string;
		source: string;
		published_at: string;
		is_read: boolean;
	};
	type MuteWord = { id: number; word: string };

	let articles = $state<Article[]>([]);
	let muteWords = $state<MuteWord[]>([]);
	let newMuteWord = $state('');

	let page = $state(1);
	let hasMore = $state(true);

	const API_BASE = 'http://127.0.0.1:3000/api';

	function getBadgeStyle(source: string) {
		if (source === 'Zenn') return 'background: #3EA8FF; color: white;';
		if (source === 'Qiita') return 'background: #55C500; color: white;';
		if (source === 'Hatena Blog') return 'background: #00A4DE; color: white;';
		if (source === 'ZDNet') return 'background: #CC0000; color: white;';
		return 'background: #666666; color: white;';
	}

	async function fetchArticles(reset = false) {
		if (reset) {
			page = 1;
			articles = [];
			hasMore = true;
		}
		if (!hasMore) return;

		const res = await fetch(`${API_BASE}/articles?page=${page}`);
		const newData: Article[] = await res.json();

		if (newData.length < 50) {
			hasMore = false;
		}
		articles = [...articles, ...newData];
	}

	async function loadMore() {
		page += 1;
		await fetchArticles();
	}

	async function markAsRead(article: Article, e?: MouseEvent) {
		if (article.is_read) return; // 既に既読なら何もしない（通常のリンク遷移になる）

		if (e) e.preventDefault();
		await fetch(`${API_BASE}/articles/${article.id}/read`, { method: 'POST' });

		article.is_read = true;
		window.open(article.url, '_blank');
	}

	async function fetchMuteWords() {
		const res = await fetch(`${API_BASE}/mutewords`);
		muteWords = await res.json();
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
		fetchArticles(true);
	}

	async function removeMuteWord(id: number) {
		await fetch(`${API_BASE}/mutewords/${id}`, { method: 'DELETE' });
		await fetchMuteWords();
		fetchArticles(true);
	}

	onMount(() => {
		fetchArticles(true);
		fetchMuteWords();
	});
</script>

<main style="max-width: 800px; margin: 0 auto; padding: 1rem; font-family: sans-serif;">
	<header style="margin-bottom: 2rem;">
		<h1 style="margin: 0;">マイニュース</h1>
	</header>

	<details
		style="margin-bottom: 2rem; background: #f8f9fa; border: 1px solid #dee2e6; border-radius: 8px; padding: 0.5rem 1rem;"
	>
		<summary style="cursor: pointer; font-weight: bold; color: #495057; outline: none;"
			>⚙️ ミュート設定</summary
		>
		<div style="margin-top: 1rem;">
			<div style="display: flex; gap: 0.5rem; margin-bottom: 1rem;">
				<input
					type="text"
					bind:value={newMuteWord}
					placeholder="ミュートする単語を入力"
					style="flex: 1; padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px;"
				/>
				<button
					onclick={addMuteWord}
					style="padding: 0.5rem 1rem; cursor: pointer; background: #333; color: white; border: none; border-radius: 4px;"
					>追加</button
				>
			</div>
			<div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
				{#each muteWords as mw}
					<span
						style="background: #e9ecef; padding: 4px 8px; border-radius: 4px; font-size: 0.9rem; display: flex; align-items: center; gap: 0.5rem; border: 1px solid #ced4da;"
					>
						{mw.word}
						<button
							onclick={() => removeMuteWord(mw.id)}
							style="background: none; border: none; color: #dc3545; cursor: pointer; padding: 0; font-size: 1.1rem; line-height: 1;"
							>×</button
						>
					</span>
				{/each}
			</div>
		</div>
	</details>

	<ul style="list-style: none; padding: 0;">
		{#each articles as article}
			<li
				style="margin-bottom: 1rem; border-bottom: 1px solid #eee; padding-bottom: 0.5rem; display: flex; align-items: flex-start; justify-content: space-between; transition: opacity 0.2s; opacity: {article.is_read
					? '0.4'
					: '1'};"
			>
				<div>
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
						onclick={(e) => markAsRead(article, e)}
						target="_blank"
						style="font-size: 1.1rem; color: #0056b3; text-decoration: none; font-weight: bold; line-height: 1.4;"
					>
						{article.title}
					</a>
				</div>
			</li>
		{/each}
	</ul>

	{#if hasMore}
		<div style="text-align: center; margin-top: 2rem;">
			<button
				onclick={loadMore}
				style="padding: 0.8rem 2rem; font-size: 1rem; cursor: pointer; background: #eee; border: none; border-radius: 20px;"
			>
				さらに読み込む
			</button>
		</div>
	{:else if articles.length > 0}
		<p style="text-align: center; color: #666; margin-top: 2rem;">すべての記事を読み込みました</p>
	{/if}
</main>
