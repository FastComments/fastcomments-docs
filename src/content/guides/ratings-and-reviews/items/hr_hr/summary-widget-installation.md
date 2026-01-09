Ispod je Vanilla JS kod za instalaciju Summary Widgeta. React biblioteka također ima ovaj widget.

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

Widget će automatski pronaći pitanja koja treba prikazati u sažetku na temelju odgovarajuće konfiguracije widgeta za tu stranicu/sajt.

Ako trebate taj widget u nekoj od naših drugih biblioteka koje ga još nemaju, otvorite zahtjev za podršku kako bismo znali da ga dodamo.