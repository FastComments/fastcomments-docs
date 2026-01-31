コメントウィジェット用のすべてのライブラリ（現在は Angular、React、Vue）はコールバックをサポートしています。

コールバックは設定オブジェクトで指定し、各ライブラリで同じシグネチャを持ちます。

サポートされているコールバックは以下です：

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

正確なシグネチャは[TypeScript 定義](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124)で確認できます。

以下はすべてのコールバックを使用した例です：

[inline-code-attrs-start title = 'コールバックの例'; inline-code-attrs-end]
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
            // return true; // デフォルトの動作（fastcomments.com のユーザープロファイルを開く）を防ぐには true を返します。
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // 非同期動作をエミュレートしています（API 呼び出し等）。
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // comments はページのデフォルトのソート順で並べられます。デフォルトのソートは Most Relevant（例：投票数が多いもの等）や Newest First である場合があります。
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    }];
</script>
[inline-code-end]