Her er en lille demo af resumé-widget'en:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### Cachelagring

Bemærk, at resuméerne caches i 30 sekunder, eller fem minutter hvis der er et stort antal anmeldelser. På grund af dette vises din anmeldelse muligvis ikke med det samme i resuméet, men det gør det muligt for os at sænke omkostningerne ved at levere resumé-widget'en.