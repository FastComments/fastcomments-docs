---
Мы можем настроить FastComments так, чтобы уведомлять Google Аналитику 4, когда кто-то взаимодействует с виджетом комментариев.

Мы можем отслеживать, когда пользователи:

- Оставляют комментарий.
- Голосуют.

Вот пример фрагмента кода, который делает это:

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

Это добавит два события:

- Метка: `Comment Posted`, Категория: `Engagement`, ID: `post_comment`
- Метка: `User Voted on a Comment`, Категория: `Engagement`, ID: `vote_comment`

---