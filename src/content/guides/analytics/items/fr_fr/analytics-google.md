Nous pouvons configurer FastComments pour notifier Google Analytics 4 lorsqu'une personne interagit avec le widget de commentaires.

Nous pouvons suivre quand les utilisateurs :

- Commentent.
- Votent.

Voici un extrait de code pour cela :

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

Cela ajoutera deux événements :

- Libellé : `Comment Posted`, Catégorie : `Engagement`, ID : `post_comment`
- Libellé : `User Voted on a Comment`, Catégorie : `Engagement`, ID : `vote_comment`