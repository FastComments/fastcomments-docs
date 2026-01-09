---
다음은 요약 위젯을 설치하기 위한 Vanilla JS 코드입니다. React 라이브러리에도 이 위젯이 포함되어 있습니다.

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

이 위젯은 해당 페이지/사이트의 위젯 구성에 따라 요약에 표시할 질문들을 자동으로 찾습니다.

다른 라이브러리들 중 위젯이 포함되어 있지 않은 곳에 이 위젯이 필요하다면, 추가할 수 있도록 지원 티켓을 제출해 주세요.

---