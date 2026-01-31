FastComments を設定して、コメントウィジェットでの操作があった際に Google アナリティクス 4 に通知することができます。

ユーザーが次のような操作を行ったときに追跡できます:

- コメント.
- 投票.

以下はそのためのコードスニペットの例です:

[inline-code-attrs-start title = 'Google アナリティクス 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
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
    }];
</script>
[inline-code-end]

これにより、2つのイベントが追加されます:

- ラベル: `Comment Posted`, カテゴリ: `Engagement`, ID: `post_comment`
- ラベル: `User Voted on a Comment`, カテゴリ: `Engagement`, ID: `vote_comment`