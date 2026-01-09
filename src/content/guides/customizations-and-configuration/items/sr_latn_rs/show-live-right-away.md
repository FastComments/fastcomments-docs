[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Po defaultu, komentarisanje u realnom vremenu je omogućeno. To znači da ako se bilo koji komentari dodaju, brišu, uređuju ili prikvače, promene bi trebalo da se pojave
kod svih korisnika koji u isto vreme gledaju nit komentara.

Međutim, po defaultu ti novi komentari će se pojaviti ispod dinamički prikazanog dugmeta sa tekstom sličnim "Prikaži 2 nova komentara".

Ako su novi komentari odgovori direktno na stranicu, dugme će se prikazati na vrhu niti komentara. Ako su odgovori na određeni komentar, dugme će se prikazati ispod tog komentara.

Ovo služi da se spreči stalna promena veličine stranice kod korisnika, što može izazvati frustraciju prilikom pokušaja da se uhvati traka za skrolovanje.

Za neke slučajeve upotrebe, kao što su licitacije uživo ili online događaji, ovo nije poželjno ponašanje - možda ćete želeti da widget za komentare bude više kao "čet" kutija gde se novi komentari "prikažu odmah".

Stoga, ime zastavice koja omogućava tu funkciju: **showLiveRightAway**.

Možemo je uključiti na sledeći način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---