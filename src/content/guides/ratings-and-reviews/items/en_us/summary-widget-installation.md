Below is the Vanilla JS code for installing the Summary Widget. The React library also includes this widget.

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

The widget will automatically find the questions to display in the summary based on the corresponding widget configuration for that page/site.

If you need the widget in one of our other libraries that doesn't include it, file a support ticket so we know to add it.

---