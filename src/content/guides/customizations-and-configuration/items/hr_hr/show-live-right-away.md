[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, uživo komentiranje je omogućeno. To znači da ako se bilo koji komentar doda, izbriše, uredi ili zakači, promjene bi se trebale pojaviti svim korisnicima koji pregledavaju nit komentara u isto vrijeme.

Međutim, prema zadanim postavkama ti novi komentari će se pojaviti ispod dinamički prikazanog gumba s tekstom sličnim "Show 2 New Comments".

Ako su novi komentari odgovori izravno na stranicu, gumb će se prikazati na vrhu nite komentara. Ako su odgovori na određeni komentar, gumb će se prikazati ispod tog komentara.

Ovo je kako bi se spriječilo stalno mijenjanje veličine stranice za korisnika, što bi moglo izazvati frustraciju prilikom pokušaja hvatanja klizača.

Za neke slučajeve upotrebe, poput uživo licitiranja ili online događaja, ovo nije željeno ponašanje - možda želite da widget za komentiranje bude više poput "chat" okvira gdje se novi komentari "prikazuju odmah".

Stoga, naziv zastavice koja omogućuje tu značajku: **showLiveRightAway**.

Možemo ga uključiti na sljedeći način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Prikaži komentare uživo odmah'; code-example-end]

Ovo se može prilagoditi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Postavka za skrivanje uživo komentara prebačena tako da se novi komentari pojavljuju odmah umjesto iza gumba'; title='Prikaži komentare uživo odmah' app-screenshot-end]