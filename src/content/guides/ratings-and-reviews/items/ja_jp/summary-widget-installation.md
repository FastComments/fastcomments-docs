---
以下はサマリーウィジェットをインストールするためのVanilla JSのコードです。Reactライブラリにもこのウィジェットがあります。

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

ウィジェットは、そのページ/サイトに対応するウィジェットの設定に基づいて、サマリーに表示する質問を自動的に検出します。

当社の他のライブラリにこのウィジェットが含まれていないが必要な場合は、追加のためにサポートチケットを提出してください。

---