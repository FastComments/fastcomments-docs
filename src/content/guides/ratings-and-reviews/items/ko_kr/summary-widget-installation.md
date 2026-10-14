아래는 요약 위젯을 설치하기 위한 Vanilla JS 코드입니다. React 라이브러리에도 이 위젯이 포함되어 있습니다.

[inline-code-attrs-start title = '요약 위젯 설치'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

위젯은 해당 페이지/사이트에 대한 위젯 구성에 따라 요약에 표시할 질문을 자동으로 찾습니다.

### Filtering Comments by Rating

같은 페이지에 댓글 위젯도 포함되어 있고, 각 댓글 작성자의 답변을 댓글에 표시하도록 설정된 경우, 세부 항목의 행이 클릭 가능해집니다. 예를 들어 "5" 행을 클릭하면 해당 답변을 선택한 댓글만 표시하도록 댓글 위젯이 필터링됩니다. 행을 다시 클릭하거나 댓글 위젯의 "Showing only" 칩에 있는 X를 클릭하면 모든 댓글이 다시 표시됩니다. 별도의 설정이 필요하지 않으며, 요약 위젯은 동일한 `tenantId`와 `urlId`를 가진 댓글 위젯을 찾아냅니다.

이는 별점, 엄지손가락, 이모지, 척도 및 단일 선택 질문에 모두 적용됩니다. 댓글 박스 위에 댓글 위젯이 표시하는 세부 항목도 동일한 방식으로 클릭할 수 있습니다.

다른 라이브러리 중 위젯이 포함되지 않은 경우, 추가 요청을 위해 지원 티켓을 제출해 주세요.