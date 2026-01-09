Yorum widget'ı için tüm kütüphaneler (şu anda Angular, React, Vue) geri çağırmaları destekler.

Geri çağırmalar, her kütüphane için aynı imzaya sahip yapılandırma nesnesinde belirtilir.

Desteklenen geri çağırmalar:

- onInit
- onAuthenticationChange
- onRender
- commentCountUpdated
- onReplySuccess
- onVoteSuccess
- onImageClicked
- onOpenProfile
- onCommentSubmitStart
- onCommentsRendered

Tam imzaları şu [TypeScript tanımlarında](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124) bulabilirsiniz.

İşte tüm geri çağırmaların kullanıldığı bir örnek:

[inline-code-attrs-start title = 'Geri Çağırma Örnekleri'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: 'demo',
        onInit: function () {
            console.log('Library started to fetch comments!');
        },
        onAuthenticationChange: function (eventName, userObj) {
            console.log('User authenticated!', eventName, userObj);
        },
        onRender: function () {
            console.log('Render event happened!');
        },
        commentCountUpdated: function (newCount) {
            console.log('New comment count:', newCount);
        },
        onReplySuccess: function (comment) {
            console.log('New comment saved!', comment);
        },
        onVoteSuccess: function (comment, voteId, direction, status) {
            console.log('New vote saved!', comment, voteId, direction, status);
        },
        onImageClicked: function (src) {
            console.log('Image clicked!', src);
        },
        onOpenProfile: function (userId) {
            console.log('User tried to open profile', userId);
            // return true; // varsayılan davranışı önlemek için true döndürün (fastcomments.com kullanıcı profilinin açılmasını).
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // asenkron davranışı taklit etme (API çağırma vb.).
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // yorumlar sayfadaki varsayılan sıralamaya göre sıralanır; bu 'Most Relevant' (ör: en çok oy alanlar vb.) veya 'Newest First' olabilir
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    });
</script>
[inline-code-end]