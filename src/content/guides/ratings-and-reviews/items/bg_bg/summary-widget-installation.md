Below is the Vanilla JS code for installing the Summary Widget. The React library also has this widget.

[inline-code-attrs-start title = 'Инсталиране на Summary Widget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

### Filtering Comments by Rating

When the comment widget for the same page is also on the page, and it is configured to show each commenter's answer on their comment, the rows of the breakdown become clickable. Clicking a row, for example "5", filters the comment widget to the comments that gave that answer. Clicking the row again, or the X on the "Showing only" chip in the comment widget, shows all the comments again. This needs no configuration: the Summary Widget finds the comment widget with the same `tenantId` and `urlId`.

This works for star, thumbs, emoji, scale and single choice questions. The breakdown the comment widget shows above the comment box can be clicked the same way.

If you require the widget in one of our other libraries that does not have it, file a support ticket so that we know to add it.

---