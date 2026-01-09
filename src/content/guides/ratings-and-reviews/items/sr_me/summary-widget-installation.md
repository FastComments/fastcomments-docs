Ispod se nalazi Vanilla JS kod za instalaciju Summary Widgeta. React biblioteka takođe sadrži ovaj widget.

[inline-code-attrs-start title = 'Instalacija Summary Widgeta'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widget će automatski pronaći pitanja koja treba prikazati u rezimeu na osnovu odgovarajuće konfiguracije widgeta za tu stranicu/sajt.

Ako vam treba ovaj widget u nekoj od naših drugih biblioteka koja ga nema, otvorite tiket za podršku kako bismo znali da ga dodamo.

---