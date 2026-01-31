FastComments lahko konfiguriramo tako, da obvesti Google Analytics 4, ko nekdo sodeluje s komentarskim vtičnikom.

Lahko spremljamo, kdaj uporabniki:

- Objavijo komentar.
- Glasujejo.

Tukaj je primer odlomka kode za to:

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

To bo dodalo dva dogodka:

- Oznaka: `Comment Posted`, Kategorija: `Engagement`, ID: `post_comment`
- Oznaka: `User Voted on a Comment`, Kategorija: `Engagement`, ID: `vote_comment`