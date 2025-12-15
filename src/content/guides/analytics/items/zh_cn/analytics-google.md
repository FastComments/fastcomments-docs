我们可以配置FastComments在有人与评论小部件交互时通知Google Analytics 4。

我们可以跟踪用户何时：

- 评论。
- 投票。

以下是执行此操作的示例代码片段：

[inline-code-attrs-start title = 'Google Analytics 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        onReplySuccess: function (comment) {
            gtag('event', 'post_comment', {
                'event_category': 'Engagement',
                'event_label': 'Comment Posted'
            });
        },
        onVoteSuccess: function (comment) {
            gtag('event', 'vote_comment', {
                'event_category': 'Engagement',
                'event_label': 'User Voted on a Comment'
            });
        }
    });
</script>
[inline-code-end]

这将添加两个事件：

- 标签：`Comment Posted`，类别：`Engagement`，ID：`post_comment`
- 标签：`User Voted on a Comment`，类别：`Engagement`，ID：`vote_comment`
