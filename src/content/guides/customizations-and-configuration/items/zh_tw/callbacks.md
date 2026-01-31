所有評論小工具的函式庫（目前為 Angular、React、Vue）都支援回呼。

回呼在設定物件中指定，且每個函式庫使用相同的簽名。

支援的回呼包括：

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

精確的簽名可在 [TypeScript 定義](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124) 中找到。

以下範例示範如何使用所有回呼：

[inline-code-attrs-start title = '回呼 範例'; inline-code-attrs-end]
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
            // return true; // 傳回 true 以防止預設行為（開啟 fastcomments.com 使用者頁面）。
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // 模擬非同步行為（呼叫 API 等）。
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // comments 會依頁面上的預設排序排序，該排序可能是 Most Relevant（例如：most upvoted 等），或 Newest First。
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    }];
</script>
[inline-code-end]