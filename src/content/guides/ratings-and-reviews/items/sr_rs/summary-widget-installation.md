---
Ispod se nalazi Vanilla JS kod za instalaciju vidžeta sažetka. React biblioteka takođe sadrži ovaj vidžet.

[inline-code-attrs-start title = 'Instalacija vidžeta sažetka'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Vidžet će automatski pronaći pitanja koja treba prikazati u sažetku na osnovu odgovarajuće konfiguracije vidžeta za tu stranicu/sajt.

Ako vam je potreban vidžet u jednoj od naših drugih biblioteka koja ga nema, otvorite tiket za podršku kako bismo znali da ga dodamo.

---