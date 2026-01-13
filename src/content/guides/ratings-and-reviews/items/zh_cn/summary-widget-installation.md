下面是用于安装摘要小部件的 Vanilla JS 代码。React 库也包含此小部件。

[inline-code-attrs-start title = '摘要小部件安装'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

该小部件会根据该页面/站点对应的小部件配置，自动查找要在摘要中显示的问题。

如果您在我们其他没有该小部件的库中需要此小部件，请提交支持工单，以便我们知道需要添加它。

---