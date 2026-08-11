[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Privzeto FastComments ne sledi, kdo je ogledal vsak komentar, niti ne ponuja statistike o tem.

Vendar pa lahko to funkcijo omogočimo, in sistem bo začel slediti, ko se uporabnik pomika do komentarja.

Ko se to zgodi, se poleg ikone očesa na vsakem komentarju poveča števec. Števec se posodablja v živo in je skrajšan glede na uporabnikovo lokalno nastavitev.

To lahko omogočimo tako, da nastavimo zastavico **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Omogočanje števila ogledov komentarjev'; code-example-end]

To lahko prilagodite brez kode na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Stran za prilagajanje gradnika z označenim potrditvenim poljem za število ogledov, tako da vsak komentar prikazuje ikono očesa in število'; title='Omogočanje števila ogledov komentarjev'; app-screenshot-end]

Sledimo ID-ju uporabnika*, ki je ogledal komentar, tako da se pri ponovnem ogledu komentarja števec ne poveča. Če komentar ponovno ogledate po dveh letih, se števec poveča.

- *Opomba: ali anonimni ID seje, ali uporabnikov IP kot zgoščena vrednost.