Below is the Vanilla JS code for installing the Summary Widget. The React library also has this widget.

[inline-code-attrs-start title = 'Summary Widget Installation'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

The widget will automatically find the questions to show in the summary based on the corresponding widget configuration for that page/site.

If you require the widget in one of our other libraries that does not have it, file a support ticket so that we know to add it.
