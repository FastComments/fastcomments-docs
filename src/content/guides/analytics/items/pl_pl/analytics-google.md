Możemy skonfigurować FastComments, aby powiadamiał Google Analytics 4, gdy ktoś wchodzi w interakcję z widgetem komentarzy.

Możemy śledzić, gdy użytkownicy:

- Komentują.
- Głosują.

Oto przykładowy fragment kodu:

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

To doda dwa zdarzenia:

- Etykieta: `Comment Posted`, Kategoria: `Engagement`, ID: `post_comment`
- Etykieta: `User Voted on a Comment`, Kategoria: `Engagement`, ID: `vote_comment`
