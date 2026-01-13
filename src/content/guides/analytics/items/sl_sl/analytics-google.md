FastComments lahko konfiguriramo, da obvesti Google Analytics 4, ko nekdo komunicira s pripomočkom za komentarje.

Lahko sledimo, ko uporabniki:

- Komentirajo.
- Glasujejo.

Tukaj je primer kode za to:

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

To bo dodalo dva dogodka:

- Oznaka: `Comment Posted`, Kategorija: `Engagement`, ID: `post_comment`
- Oznaka: `User Voted on a Comment`, Kategorija: `Engagement`, ID: `vote_comment`
