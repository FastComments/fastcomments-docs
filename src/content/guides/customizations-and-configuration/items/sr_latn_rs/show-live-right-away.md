[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Podrazumevano je omogućeno live komentarisanje. To znači da, ako se bilo koji komentar doda, obriše, izmeni ili zakači, promene bi trebalo da se pojave svim korisnicima koji gledaju nit komentara u isto vreme.

Međutim, podrazumevano ti novi komentari će se pojaviti ispod dinamički prikazanog dugmeta sa tekstom sličnim “Show 2 New Comments”.

Ako su novi komentari odgovori direktno na stranicu, dugme će se prikazati na vrhu niti komentara. Ako su odgovori na određeni komentar, dugme će se prikazati ispod tog komentara.

Ovo je da se spreči stalna promena veličine stranice kod korisnika, što može izazvati frustraciju prilikom pokušaja hvatanja klizača.

Za neke slučajeve upotrebe, poput live licitiranja ili online događaja, ovo nije željeno ponašanje – možda želite da vidžet za komentarisanje bude više poput “chat” kutije gde se novi komentari “prikazuju odmah”.

Stoga, ime zastavice koja omogućava tu funkciju: **showLiveRightAway**.

Možemo je uključiti na sledeći način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagođavanje vidžeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Postavka za sakrivanje live komentara je prebačena tako da se novi komentari pojavljuju odmah umesto iza dugmeta'; title='Prikaži live komentare odmah' app-screenshot-end]

---