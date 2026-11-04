Spodaj je Vanilla JS koda za namestitev Povzetnega pripomočka (Summary Widget). Knjižnica React prav tako vsebuje ta pripomoček.

[inline-code-attrs-start title = 'Namestitev Povzetnega pripomočka'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Pripomoček bo samodejno našel vprašanja za prikaz v povzetku na podlagi ustrezne konfiguracije pripomočka za to stran/spletno mesto.

### Filtriranje komentarjev po oceni

Ko je pripomoček za komentarje za isto stran tudi na strani in je konfiguriran tako, da prikazuje odgovor komentatorja na njegovem komentarju, postanejo vrstice razčlenitve
klikljive. Klik na vrstico, na primer "5", filtrira pripomoček za komentarje na komentarje, ki so dali ta odgovor. Ponoven klik na vrstico ali na X na oznaki "Prikazujem samo"
v pripomočku za komentarje znova prikaže vse komentarje. To ne potrebuje konfiguracije: Povzetni pripomoček najde pripomoček za komentarje z istim `tenantId` in `urlId`.

To deluje za vprašanja z zvezdicami, palčki, emotikoni, lestvico in enojno izbiro. Razčlenitev, ki jo pripomoček za komentarje prikaže nad oknom za komentarje, je mogoče klikniti na enak način.

Če potrebujete pripomoček v eni od naših drugih knjižnic, ki ga nima, oddajte zahtevek za podporo, da vemo, da ga moramo dodati.