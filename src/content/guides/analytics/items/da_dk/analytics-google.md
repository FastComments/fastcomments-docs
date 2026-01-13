Vi kan konfigurere FastComments til at underrette Google Analytics 4, når nogen interagerer med kommentar-widgetten.

Vi kan spore, når brugere:

- Kommenterer.
- Stemmer.

Her er et eksempel på kode til det:

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

Dette vil tilføje to begivenheder:

- Label: `Comment Posted`, Kategori: `Engagement`, ID: `post_comment`
- Label: `User Voted on a Comment`, Kategori: `Engagement`, ID: `vote_comment`
