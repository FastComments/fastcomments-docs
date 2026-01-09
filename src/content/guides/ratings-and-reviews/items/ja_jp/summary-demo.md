以下はサマリウィジェットの小さなデモです：

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // クエリパラメータを削除
    });
</script>

### キャッシュ

サマリーは30秒間キャッシュされることに注意してください。レビューが大量にある場合は5分間キャッシュされます。このため、あなたのレビューがサマリーにすぐ表示されない場合がありますが、これ
によりサマリウィジェットの配信コストを下げることができます。