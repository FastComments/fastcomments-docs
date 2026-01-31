Wir können FastComments so konfigurieren, dass Google Analytics 4 benachrichtigt wird, wenn jemand mit dem Kommentar-Widget interagiert.

Wir können nachverfolgen, wenn Nutzer:

- Kommentieren.
- Abstimmen.

Hier ein Beispiel-Code-Snippet dafür:

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

Dadurch werden zwei Ereignisse hinzugefügt:

- Bezeichnung: `Comment Posted`, Kategorie: `Engagement`, ID: `post_comment`
- Bezeichnung: `User Voted on a Comment`, Kategorie: `Engagement`, ID: `vote_comment`

---