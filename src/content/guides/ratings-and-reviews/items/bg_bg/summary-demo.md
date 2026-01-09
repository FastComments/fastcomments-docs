Ето едно малко демо на уиджета за обобщение:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // премахва параметрите на заявката
    });
</script>

### Кеширане

Имайте предвид, че обобщенията се кешират за 30 секунди, или пет минути ако има голям брой отзиви. Поради това вашият отзив може да не се появи веднага в обобщението, но това
ни позволява да намалим разходите за обслужване на уиджета за обобщение.