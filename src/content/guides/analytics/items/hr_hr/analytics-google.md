Možemo konfigurirati FastComments da obavijesti Google Analytics 4 kada netko stupi u interakciju s widgetom za komentare.

Možemo pratiti kada korisnici:

- Komentiraju.
- Glasaju.

Evo primjera koda za to:

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

Ovo će dodati dva događaja:

- Oznaka: `Comment Posted`, Kategorija: `Engagement`, ID: `post_comment`
- Oznaka: `User Voted on a Comment`, Kategorija: `Engagement`, ID: `vote_comment`

---