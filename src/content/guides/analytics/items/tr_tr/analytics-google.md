FastComments'i, birisi yorum widget'ı ile etkileşime geçtiğinde Google Analytics 4'ü bilgilendirecek şekilde yapılandırabiliriz.

Kullanıcıların ne zaman:

- Yorum yaptığını.
- Oy verdiğini izleyebiliriz.

İşte bunun için örnek bir kod parçacığı:

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

Bu iki olay ekleyecektir:

- Etiket: `Comment Posted`, Kategori: `Engagement`, ID: `post_comment`
- Etiket: `User Voted on a Comment`, Kategori: `Engagement`, ID: `vote_comment`
