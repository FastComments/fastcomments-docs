---
Spodaj je koda Vanilla JS za namestitev pripomočka povzetka. Knjižnica React ima tudi ta pripomoček.

[inline-code-attrs-start title = 'Namestitev pripomočka povzetka'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Pripomoček bo samodejno našel vprašanja, ki jih je treba prikazati v povzetku, na podlagi ustrezne konfiguracije pripomočka za to stran/spletno mesto.

Če potrebujete pripomoček v eni izmed naših drugih knjižnic, ki ga nima, odprite podporno vozovnico, da bomo vedeli, da ga moramo dodati.

---