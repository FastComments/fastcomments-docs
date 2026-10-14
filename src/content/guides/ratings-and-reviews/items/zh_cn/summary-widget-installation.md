下面是用于安装摘要小部件的 Vanilla JS 代码。React 库也提供此小部件。

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

小部件会根据该页面/站点的相应小部件配置，自动查找要在摘要中显示的问题。

### Filtering Comments by Rating

当同一页面上也存在评论小部件，并且它被配置为在评论中显示每个评论者的答案时，细分表的行会变为可点击。点击某一行，例如 “5”，会将评论小部件过滤为给出该答案的评论。再次点击该行，或点击评论小部件中 “仅显示” 芯片上的 X，则会重新显示所有评论。此功能无需任何配置：摘要小部件会根据相同的 `tenantId` 和 `urlId` 找到对应的评论小部件。

此功能适用于星级、拇指、表情符号、量表和单选问题。评论小部件在评论框上方显示的细分同样可以以相同方式点击。

如果您需要在我们其他库中使用此小部件但该库尚未提供，请提交支持工单，以便我们知晓并添加该功能。