以下はサマリーウィジェットをインストールするための Vanilla JS コードです。React ライブラリにもこのウィジェットがあります。

[inline-code-attrs-start title = 'サマリーウィジェットのインストール'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

ウィジェットは、ページ/サイトに対応するウィジェット設定に基づいて、サマリーに表示する質問を自動的に検出します。

### 評価でコメントをフィルタリング

同じページにコメントウィジェットも配置されていて、各コメント投稿者の回答をコメントに表示するよう設定されている場合、内訳の行がクリック可能になります。たとえば「5」の行をクリックすると、その回答を付けたコメントだけがコメントウィジェットに表示されます。行を再度クリックするか、コメントウィジェット内の「Showing only」チップの X をクリックすると、すべてのコメントが再び表示されます。これは設定不要で、サマリーウィジェットは同じ `tenantId` と `urlId` を持つコメントウィジェットを自動的に検出します。

これは星評価、サムズ、絵文字、スケール、単一選択の質問すべてで機能します。コメントボックス上部にコメントウィジェットが表示する内訳も同様にクリックできます。

他のライブラリでこのウィジェットがまだ提供されていない場合は、サポートチケットを提出してください。追加が必要であることを把握できます。