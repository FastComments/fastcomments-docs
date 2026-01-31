---
댓글 위젯용 모든 라이브러리(현재 Angular, React, Vue)는 콜백을 지원합니다.

콜백은 구성 객체(configuration object)에 지정되며, 각 라이브러리에서 동일한 시그니처를 가집니다.

지원되는 콜백은:

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

정확한 시그니처는 [TypeScript 정의](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124)에서 확인할 수 있습니다.

다음은 모든 콜백을 사용한 예제입니다:

[inline-code-attrs-start title = '콜백 예제'; inline-code-attrs-end]
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
            // return true; // 기본 동작(fastcomments.com 사용자 프로필 열기)을 방지하려면 true를 반환하세요.
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // 비동기 동작(예: API 호출 등)을 에뮬레이트합니다.
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // comments는 페이지의 기본 정렬에 따라 정렬되며, 이는 Most Relevant(예: 가장 많은 추천을 받은 항목 등) 또는 Newest First(최신순)일 수 있습니다
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    }];
</script>
[inline-code-end]


---