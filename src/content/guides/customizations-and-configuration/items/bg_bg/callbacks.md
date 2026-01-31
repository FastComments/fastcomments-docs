Всички библиотеки за коментарния уиджет (в момента Angular, React, Vue) поддържат callback-и.

Callback-ите се задават в конфигурационния обект, със същия подпис за всяка библиотека.

Поддържаните callback-и са:

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

Точните подписи могат да бъдат намерени в [TypeScript дефинициите](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124).

Ето пример, в който се използват всички callback-и:

[inline-code-attrs-start title = 'Примери за callback-и'; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
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
            // return true; // върнете true, за да предотвратите подразбиращото се поведение (отворяне на потребителски профил в fastcomments.com).
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // емулиране на асинхронно поведение (извикване на API и т.н.).
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // comments са сортирани по подразбиращото се сортиране на страницата, което може да бъде Most Relevant (напр.: most upvoted и т.н.) или Newest First
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    }];
</script>
[inline-code-end]