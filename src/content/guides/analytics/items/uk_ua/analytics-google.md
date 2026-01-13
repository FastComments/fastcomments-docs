Ми можемо налаштувати FastComments для сповіщення Google Analytics 4, коли хтось взаємодіє з віджетом коментарів.

Ми можемо відстежувати, коли користувачі:

- Коментують.
- Голосують.

Ось приклад коду для цього:

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

Це додасть дві події:

- Мітка: `Comment Posted`, Категорія: `Engagement`, ID: `post_comment`
- Мітка: `User Voted on a Comment`, Категорія: `Engagement`, ID: `vote_comment`
