這裡有一個摘要小工具的小示範：

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### 快取

請注意，摘要會被快取 30 秒；如果評論數量很多則為五分鐘。由於此一原因，您的評論可能不會立即顯示在摘要中，但這
讓我們能降低提供摘要小工具的成本。