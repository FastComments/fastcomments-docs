Мы можем настроить FastComments для уведомления Google Analytics 4, когда кто-то взаимодействует с виджетом комментариев.

Мы можем отслеживать, когда пользователи:

- Комментируют.
- Голосуют.

Вот пример кода для этого:

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

Это добавит два события:

- Метка: `Comment Posted`, Категория: `Engagement`, ID: `post_comment`
- Метка: `User Voted on a Comment`, Категория: `Engagement`, ID: `vote_comment`
