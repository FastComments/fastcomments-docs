以下是安装摘要小部件（Summary Widget）的 Vanilla JS 代码。React 库中也有这个小部件。

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

该小部件会根据对应页面/站点的小部件配置，自动找到要在摘要中显示的问题。

### 按评分筛选评论

当同一页面的评论小部件也在页面上，并且其配置为在评论者的评论上显示其回答时，细分统计的各行将变为可点击。点击某一行（例如“5”），会将评论小部件筛选为给出该回答的评论。再次点击该行，或点击评论小部件中“仅显示”标签上的 X，即可重新显示所有评论。这无需任何配置：摘要小部件会自动找到具有相同 `tenantId` 和 `urlId` 的评论小部件。

这适用于星级、点赞、表情、量表和单选题。评论小部件在评论框上方显示的细分统计也可以以同样的方式点击。

如果您需要在我们的其他库中使用该小部件但该库尚不支持，请提交支持工单，以便我们知道要添加它。