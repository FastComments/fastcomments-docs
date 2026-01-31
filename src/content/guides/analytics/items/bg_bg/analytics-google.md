Можем да конфигурираме FastComments да уведомява Google Аналитика 4, когато някой взаимодейства с уиджета за коментари.

Можем да проследяваме кога потребителите:

- Коментират.
- Гласуват.

Ето примерен фрагмент от код за това:

[inline-code-attrs-start title = 'Google Аналитика 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
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

Това ще добави две събития:

- Етикет: `Comment Posted`, Категория: `Engagement`, ID: `post_comment`
- Етикет: `User Voted on a Comment`, Категория: `Engagement`, ID: `vote_comment`

---