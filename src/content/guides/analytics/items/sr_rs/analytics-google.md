Можемо да конфигуришемо FastComments да обавести Google Analytics 4 када неко интерагује са коментарским виџетом.

Можемо да пратимо када корисници:

- Коментаришу.
- Гласају.

Ево примера кода за то:

[inline-code-attrs-start title = 'Google Analytics 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
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

Ово ће додати два догађаја:

- Ознака: `Comment Posted`, Категорија: `Engagement`, ИД: `post_comment`
- Ознака: `User Voted on a Comment`, Категорија: `Engagement`, ИД: `vote_comment`