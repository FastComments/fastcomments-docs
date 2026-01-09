[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Podrazumevano je omogućeno komentarisanje uživo. To znači da, ako se bilo koji komentari dodaju, brišu, uređuju ili prikače, promene bi trebalo da se pojave
svim korisnicima koji u tom trenutku gledaju nit komentara.

Međutim, po defaultu ti novi komentari će se pojaviti ispod dinamički prikazanog dugmeta sa tekstom sličnim "Prikaži 2 nova komentara".

Ako su novi komentari odgovori direktno na stranicu, dugme će se prikazati na vrhu niti komentara. Ako su odgovori na određeni komentar, 
dugme će se prikazati ispod tog komentara.

Ovo je da bi se sprečilo stalno menjanje veličine stranice, što može izazvati frustraciju kada korisnik pokuša da uhvati traku za pomeranje.

Za neke slučajeve upotrebe, kao što su licitacije uživo ili onlajn događaji, ovo nije poželjno ponašanje - možda ćete želeti da widget za komentare bude
više kao "chat" prozor gde se novi komentari "prikazuju odmah".

Otuda i ime zastavice koja omogućava tu opciju: **showLiveRightAway**.

Možemo ga uključiti na sledeći način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Prikaži komentare uživo odmah'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Prikaži komentare uživo odmah' app-screenshot-end]

---