We kunnen FastComments configureren om Google Analytics 4 te informeren wanneer iemand interactie heeft met de commentaarwidget.

We kunnen bijhouden wanneer gebruikers:

- Reageren.
- Stemmen.

Hier is een voorbeeldcodefragment om dat te doen:

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

Dit voegt twee gebeurtenissen toe:

- Label: `Comment Posted`, Categorie: `Engagement`, ID: `post_comment`
- Label: `User Voted on a Comment`, Categorie: `Engagement`, ID: `vote_comment`

---