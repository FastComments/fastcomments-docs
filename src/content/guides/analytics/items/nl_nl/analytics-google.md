We kunnen FastComments configureren om Google Analytics 4 te notificeren wanneer iemand interactie heeft met de reactie-widget.

We kunnen bijhouden wanneer gebruikers:

- Reageren.
- Stemmen.

Hier is een voorbeeld codefragment hiervoor:

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

Dit voegt twee gebeurtenissen toe:

- Label: `Comment Posted`, Categorie: `Engagement`, ID: `post_comment`
- Label: `User Voted on a Comment`, Categorie: `Engagement`, ID: `vote_comment`
