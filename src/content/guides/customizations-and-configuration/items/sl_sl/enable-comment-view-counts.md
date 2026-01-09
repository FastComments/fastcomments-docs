[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Privzeto FastComments ne spremlja, kdo je ogledal posamezen komentar, niti ne nudi kakršnekoli statistike o tem.

Vendar lahko to funkcijo omogočimo, in sistem bo začel slediti, ko se vsak uporabnik pomakne do komentarja.

Ko se to zgodi, se bo ob ikoni očesa, prikazani pri vsakem komentarju, povečal števec. Števec se posodablja v živo in je skrajšan glede na lokalne nastavitve uporabnika.

To lahko omogočite z nastavitvijo zastavice **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

To je mogoče prilagoditi brez kode, na strani za prilagajanje vtičnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Spremljamo id uporabnika*, ki je ogledal komentar, tako da se ob ponovnem ogledu komentarja števec ne poveča. Če komentar ponovno ogledate
po dveh letih, se bo števec znova povečal.

- *Opomba: ali anon id seje, ali IP naslov uporabnika kot zgoščena vrednost.