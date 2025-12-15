FastCommentsを設定して、誰かがコメントウィジェットとやり取りしたときにGoogle Analytics 4に通知することができます。

ユーザーが以下を行ったときに追跡できます:

- コメント。
- 投票。

以下はそのためのコード例です:

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

これにより2つのイベントが追加されます:

- ラベル: `Comment Posted`、カテゴリ: `Engagement`、ID: `post_comment`
- ラベル: `User Voted on a Comment`、カテゴリ: `Engagement`、ID: `vote_comment`
