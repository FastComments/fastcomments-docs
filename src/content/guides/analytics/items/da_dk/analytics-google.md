Vi kan konfigurere FastComments til at underrette Google Analytics 4, når nogen interagerer med kommentar-widgeten.

Vi kan spore, når brugere:

- Kommenterer.
- Stemmer.

Her er et eksempel på et kodeudsnit, der gør det:

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

Dette vil tilføje to begivenheder:

- Etiket: `Comment Posted`, Kategori: `Engagement`, ID: `post_comment`
- Etiket: `User Voted on a Comment`, Kategori: `Engagement`, ID: `vote_comment`

---