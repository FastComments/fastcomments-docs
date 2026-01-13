Podemos configurar FastComments para notificar a Google Analytics 4 cuando alguien interactúa con el widget de comentarios.

Podemos rastrear cuando los usuarios:

- Comentan.
- Votan.

Aquí hay un ejemplo de código para hacer eso:

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

Esto agregará dos eventos:

- Etiqueta: `Comment Posted`, Categoría: `Engagement`, ID: `post_comment`
- Etiqueta: `User Voted on a Comment`, Categoría: `Engagement`, ID: `vote_comment`
