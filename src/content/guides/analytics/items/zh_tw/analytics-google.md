我們可以配置FastComments在有人與評論小工具互動時通知Google Analytics 4。

我們可以追蹤使用者何時：

- 評論。
- 投票。

以下是執行此操作的示例程式碼片段：

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

這將新增兩個事件：

- 標籤：`Comment Posted`，類別：`Engagement`，ID：`post_comment`
- 標籤：`User Voted on a Comment`，類別：`Engagement`，ID：`vote_comment`
