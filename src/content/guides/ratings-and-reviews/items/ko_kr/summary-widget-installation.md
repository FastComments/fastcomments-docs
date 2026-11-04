아래는 Summary Widget을 설치하기 위한 Vanilla JS 코드입니다. React 라이브러리에도 이 위젯이 포함되어 있습니다.

[inline-code-attrs-start title = 'Summary Widget 설치'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

위젯은 해당 페이지/사이트에 대한 위젯 설정을 기반으로 요약에 표시할 질문을 자동으로 찾습니다.

### 평점별로 댓글 필터링하기

같은 페이지에 댓글 위젯이 함께 있고, 각 댓글 작성자의 답변이 댓글에 표시되도록 설정된 경우, 분석 breakdown의 행을 클릭할 수 있게 됩니다. 예를 들어 "5" 행을 클릭하면 해당 답변을 준 댓글만 표시되도록 댓글 위젯이 필터링됩니다. 행을 다시 클릭하거나 댓글 위젯의 "Showing only" 칩에 있는 X를 클릭하면 모든 댓글이 다시 표시됩니다. 별도의 설정이 필요하지 않습니다: Summary Widget이 동일한 `tenantId`와 `urlId`를 가진 댓글 위젯을 자동으로 찾습니다.

이 기능은 별점(star), 좋아요/싫어요(thumbs), 이모지(emoji), 척도(scale), 단일 선택(single choice) 질문에 적용됩니다. 댓글 위젯이 댓글 입력란 위에 표시하는 breakdown도 같은 방식으로 클릭할 수 있습니다.

다른 라이브러리에서 이 위젯이 필요하지만 아직 제공되지 않는 경우, 지원 티켓을 제출해 주시면 추가하도록 하겠습니다.