---
Podemos configurar o FastComments para notificar o Google Analytics 4 quando alguém interagir com o widget de comentários.

Podemos rastrear quando os usuários:

- Comentam.
- Votam.

Aqui está um exemplo de trecho de código para fazer isso:

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

Isso adicionará dois eventos:

- Rótulo: `Comment Posted`, Categoria: `Engagement`, ID: `post_comment`
- Rótulo: `User Voted on a Comment`, Categoria: `Engagement`, ID: `vote_comment`

---