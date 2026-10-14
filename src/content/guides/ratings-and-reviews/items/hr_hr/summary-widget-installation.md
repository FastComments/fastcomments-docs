Dolje je Vanilla JS kod za instalaciju Summary Widgeta. React biblioteka također ima ovaj widget.

[inline-code-attrs-start title = 'Instalacija widgeta sažetka'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widget će automatski pronaći pitanja koja će prikazati u sažetku na temelju odgovarajuće konfiguracije widgeta za tu stranicu/web mjesto.

### Filtriranje komentara po ocjeni

Kada je widget za komentare za istu stranicu također prisutan na stranici i konfiguriran je da prikazuje odgovor svakog komentatora uz njihov komentar, redci razlaganja postaju klikabilni. Klikom na red, na primjerice „5“, filtrira se widget za komentare na komentare koji su dali taj odgovor. Ponovnim klikom na red ili na X na čipu „Prikazivanje samo“ u widgetu za komentare, prikazuju se svi komentari ponovno. Ovo ne zahtijeva dodatnu konfiguraciju: Summary Widget pronalazi widget za komentare s istim `tenantId` i `urlId`.

Ovo funkcionira za pitanja s ocjenjivanjem zvjezdicama, palcem, emojijima, skalom i jednim izborom. Razlaganje koje widget za komentare prikazuje iznad okvira za komentar može se kliknuti na isti način.

Ako vam je widget potreban u jednoj od naših drugih biblioteka koja ga nema, otvorite tiket za podršku kako bismo znali da ga dodamo.