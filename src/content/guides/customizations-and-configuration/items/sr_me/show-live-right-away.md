[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Podrazumevano, komentarisanje uživo je omogućeno. To znači da ako se bilo koji komentari dodaju, uklone, izmijene ili zakače, promjene bi se trebale pojaviti
svim korisnicima koji gledaju nit komentara u isto vrijeme.

Međutim, podrazumevano ti novi komentari će se pojaviti ispod dinamički prikazanog dugmeta sa tekstom sličnim "Prikaži 2 nova komentara".

Ako su novi komentari odgovori direktno stranici, dugme će se pojaviti na vrhu niti komentara. Ako su odgovori na određeni komentar, 
dugme će se pojaviti ispod tog komentara.

Ovo je da bi se spriječilo konstantno mijenjanje veličine stranice za korisnika, što može izazvati frustraciju prilikom pokušaja hvatanja trake za skrolovanje.

Za neke slučajeve upotrebe, poput licitacija uživo, ili online događaja, ovo nije željeno ponašanje - možda želite da vidžet za komentare bude
više kao "čet" kutija gdje se novi komentari "prikazuju odmah".

Dakle, naziv zastavice koja omogućava tu funkcionalnost: **showLiveRightAway**.

Možemo ga uključiti na sljedeći način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje vidžeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---