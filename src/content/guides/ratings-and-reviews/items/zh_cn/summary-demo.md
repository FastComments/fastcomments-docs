下面是摘要小部件的一个小型演示：

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### 缓存

请注意，摘要会被缓存 30 秒，如果存在大量评论则为 5 分钟。 因此，您的评论可能不会立即显示在摘要中，但这
可以降低我们提供摘要小部件的成本。

---