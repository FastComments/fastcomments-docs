Nous pouvons configurer FastComments pour notifier Google Analytics 4 lorsque quelqu'un interagit avec le widget de commentaires.

Nous pouvons suivre quand les utilisateurs:

- Commentent.
- Votent.

Voici un exemple de code pour faire cela:

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

Cela ajoutera deux événements:

- Libellé: `Comment Posted`, Catégorie: `Engagement`, ID: `post_comment`
- Libellé: `User Voted on a Comment`, Catégorie: `Engagement`, ID: `vote_comment`
