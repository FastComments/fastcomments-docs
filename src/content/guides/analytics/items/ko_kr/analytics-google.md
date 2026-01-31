---
FastComments를 구성하여 누군가 댓글 위젯과 상호작용할 때 Google 애널리틱스 4에 알림을 보낼 수 있습니다.

사용자가 다음과 같은 작업을 했을 때를 추적할 수 있습니다:

- 댓글을 작성할 때.
- 투표할 때.

다음은 그 예제 코드 스니펫입니다:

[inline-code-attrs-start title = 'Google 애널리틱스 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
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

이렇게 하면 두 개의 이벤트가 추가됩니다:

- 레이블: `Comment Posted`, 카테고리: `Engagement`, ID: `post_comment`
- 레이블: `User Voted on a Comment`, 카테고리: `Engagement`, ID: `vote_comment`

---