---
[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, komentiranje uživo je omogućeno. To znači da ako se bilo koji komentari dodaju, izbrišu, urede ili pričvrste, promjene bi se trebale pojaviti
svim korisnicima koji u istom trenutku gledaju nit komentara.

Međutim, prema zadanim postavkama ti novi komentari pojavljuju se ispod dinamički prikazanog gumba s tekstom sličnim "Prikaži 2 nova komentara".

Ako su novi komentari odgovori izravno na stranicu, gumb će se prikazati pri vrhu niti komentara. Ako su odgovori na određeni komentar, 
gumb će se prikazati ispod tog komentara.

To sprječava stalno mijenjanje veličine stranice za korisnika, što može izazvati frustraciju pri pokušaju hvatanja klizača za pomicanje.

Za neke slučajeve upotrebe, poput živog licitiranja ili online događaja, ovo nije poželjno ponašanje - možda ćete htjeti da widget za komentiranje bude
više poput "chat" okvira gdje se novi komentari "prikažu odmah".

Otud i ime zastavice koja omogućava tu značajku: **showLiveRightAway**.

Možemo ga uključiti na sljedeći način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---