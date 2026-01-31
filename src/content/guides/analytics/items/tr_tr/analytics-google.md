FastComments'i, birisi yorum bileşeniyle etkileşime geçtiğinde Google Analytics 4'e bildirim gönderecek şekilde yapılandırabiliriz.

Kullanıcıların şu eylemlerini izleyebiliriz:

- Yorum yapma.
- Oy verme.

İşte bunu yapmak için bir örnek kod parçacığı:

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

Bu iki olayı ekleyecektir:

- Etiket: `Comment Posted`, Kategori: `Engagement`, ID: `post_comment`
- Etiket: `User Voted on a Comment`, Kategori: `Engagement`, ID: `vote_comment`