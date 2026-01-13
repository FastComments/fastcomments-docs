İşte özet widget'ının küçük bir demosu:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // sorgu parametrelerini kaldır
    });
</script>

### Önbellekleme

Özetlerin 30 saniye boyunca önbelleklendiğini, çok sayıda inceleme varsa bu sürenin beş dakika olduğunu unutmayın. Bu nedenle, incelemeniz özet içinde hemen görünmeyebilir, ancak bu
özet widget'ını sunma maliyetini düşürmemize olanak tanır.