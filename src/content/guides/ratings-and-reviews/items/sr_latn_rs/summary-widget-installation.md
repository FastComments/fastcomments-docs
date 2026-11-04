Ispod je Vanilla JS kod za instalaciju Summary Widget-a. React biblioteka takođe ima ovaj vidžet.

[inline-code-attrs-start title = 'Instalacija Summary Widget-a'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Vidžet će automatski pronaći pitanja za prikaz u rezimeu na osnovu odgovarajuće konfiguracije vidžeta za tu stranicu/sajt.

### Filtriranje komentara po oceni

Kada je vidžet za komentare iste stranice takođe na stranici, i kada je konfigurisan da prikazuje odgovor svakog komentatora na njegovom komentaru, redovi u raspodeli
postaju klikabilni. Klikom na red, na primer "5", vidžet za komentare se filtrira na komentare koji su dali taj odgovor. Ponovnim klikom na red, ili na X na "Showing only" čipu
u vidžetu za komentare, ponovo se prikazuju svi komentari. Ovo ne zahteva konfiguraciju: Summary Widget pronalazi vidžet za komentare sa istim `tenantId` i `urlId`.

Ovo funkcioniše za pitanja sa zvezdicama, palcima, emodžijima, skalom i pitanja sa jednim izborom. Raspodelu koju vidžet za komentare prikazuje iznad okvira za komentar može se kliknuti na isti način.

Ako vam je potreban vidžet u jednoj od naših drugih biblioteka koja ga nema, otvorite tiket za podršku kako bismo znali da ga dodamo.