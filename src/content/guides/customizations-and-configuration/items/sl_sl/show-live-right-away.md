[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Privzeto je omogočeno komentiranje v živo. To pomeni, da se, če so komentarji dodani, izbrisani, urejeni ali pripeti, te spremembe prikažejo vsem uporabnikom, ki hkrati gledajo nit komentarjev.

Vendar pa se bodo ti novi komentarji privzeto prikazali pod dinamično prikazanim gumbom z besedilom, podobnim "Prikaži 2 nova komentarja".

Če so novi komentarji odgovori neposredno na stran, se bo gumb prikazal na vrhu niti komentarjev. Če so odgovori na določen komentar, se bo gumb prikazal pod tem komentarjem.

To preprečuje, da bi se velikost strani nenehno spreminjala za uporabnika, kar bi lahko povzročilo frustracije pri poskusu prijema drsnika.

Za nekatere primere uporabe, kot so dražbe v živo ali spletni dogodki, to ni zaželeno vedenje — morda želite, da je pripomoček za komentarje bolj podoben klepetu, kjer se novi komentarji "prikažejo takoj".

Zato se zastavica, ki omogoča to funkcijo, imenuje: **showLiveRightAway**.

Vključimo jo lahko tako:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

To je mogoče prilagoditi brez kode, na strani za prilagoditev gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]