Wir können FastComments so konfigurieren, dass Google Analytics 4 benachrichtigt wird, wenn jemand mit dem Kommentar-Widget interagiert.

Wir können verfolgen, wenn Benutzer:

- Kommentieren.
- Abstimmen.

Hier ist ein Beispiel-Code-Snippet dafür:

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

Dies fügt zwei Ereignisse hinzu:

- Label: `Comment Posted`, Kategorie: `Engagement`, ID: `post_comment`
- Label: `User Voted on a Comment`, Kategorie: `Engagement`, ID: `vote_comment`
