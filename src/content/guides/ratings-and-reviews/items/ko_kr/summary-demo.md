다음은 요약 위젯의 작은 데모입니다:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### 캐싱

요약은 30초 동안 캐시되며, 리뷰가 많은 경우에는 5분 동안 캐시된다는 점에 유의하세요. 이로 인해 귀하의 리뷰가 요약에 바로 표시되지 않을 수 있지만, 이는 요약 위젯 제공 비용을 낮추는 데 도움이 됩니다.