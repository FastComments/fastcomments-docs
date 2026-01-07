Here is a small demo of the summary widget:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### Caching

Note that the summaries are cached for 30 seconds, or five minutes if there is a large number of reviews. Due to this, so your review may not show right away in the summary, but this
allows us to lower the cost of serving the summary widget.
