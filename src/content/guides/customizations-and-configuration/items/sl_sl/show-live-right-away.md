[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Privzeto je omogočeno živo komentiranje. To pomeni, da se bodo spremembe, kot so dodajanje, brisanje, urejanje ali pripenjanje komentarjev, prikazale vsem uporabnikom, ki si ogledajo nit komentarjev, hkrati.

Vendar se bodo ti novi komentarji privzeto pojavili pod dinamično prikazanim gumbom z besedilom, podobnim "Show 2 New Comments".

Če so novi komentarji odgovori neposredno na stran, se bo gumb prikazal na vrhu nite komentarjev. Če so odgovori na določen komentar, se bo gumb prikazal pod tem komentarjem.

To je zato, da se velikost strani ne spreminja nenehno za uporabnika, kar bi lahko povzročilo frustracije pri poskusu uporabe drsnika.

Za nekatere primere uporabe, kot so živa dražba ali spletni dogodki, to ni želeno vedenje – morda želite, da je pripomoček za komentiranje bolj podoben "chat" oknu, kjer se novi komentarji "prikažejo takoj".

Zato je ime zastavice, ki omogoča to funkcijo: **showLiveRightAway**.

To lahko vklopimo na naslednji način:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

To je mogoče prilagoditi brez kode na strani za prilagajanje pripomočka:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Nastavitev za skrivanje živih komentarjev je preklopljena, tako da se novi komentarji prikažejo takoj namesto za gumbom'; title='Prikaži žive komentarje takoj' app-screenshot-end]