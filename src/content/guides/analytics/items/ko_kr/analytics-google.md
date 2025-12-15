누군가가 댓글 위젯과 상호 작용할 때 FastComments가 Google Analytics 4에 알리도록 구성할 수 있습니다.

사용자가 다음을 할 때 추적할 수 있습니다:

- 댓글.
- 투표.

다음은 이를 수행하는 예제 코드 스니펫입니다:

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

이렇게 하면 두 개의 이벤트가 추가됩니다:

- 레이블: `Comment Posted`, 카테고리: `Engagement`, ID: `post_comment`
- 레이블: `User Voted on a Comment`, 카테고리: `Engagement`, ID: `vote_comment`
