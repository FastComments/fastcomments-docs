Possiamo configurare FastComments per notificare Google Analytics 4 quando qualcuno interagisce con il widget dei commenti.

Possiamo tracciare quando gli utenti:

- Lasciano un commento.
- Votano.

Ecco un esempio di snippet di codice per farlo:

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

Questo aggiungerà due eventi:

- Etichetta: `Comment Posted`, Categoria: `Engagement`, ID: `post_comment`
- Etichetta: `User Voted on a Comment`, Categoria: `Engagement`, ID: `vote_comment`

---