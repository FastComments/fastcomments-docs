Можемо конфигурисати FastComments да обавести Google Analytics 4 када неко комуницира са виџетом за коментаре.

Можемо пратити када корисници:

- Коментаришу.
- Гласају.

Ево примера кода за то:

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

Ово ће додати два догађаја:

- Ознака: `Comment Posted`, Категорија: `Engagement`, ID: `post_comment`
- Ознака: `User Voted on a Comment`, Категорија: `Engagement`, ID: `vote_comment`
