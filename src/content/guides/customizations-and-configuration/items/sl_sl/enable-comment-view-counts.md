[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Privzeto FastComments ne sledi, kdo je ogledal vsak komentar, niti ne zagotavlja kakršnihkoli statistik o tem.

Vendar lahko to funkcijo omogočimo, nato pa bo sistem začel slediti, ko vsak uporabnik pomakne do komentarja.

Ko se to zgodi, se bo število poleg ikone očesa, prikazane na vsakem komentarju, povečalo. Število se posodablja v živo in je skrajšano glede na uporabnikovo lokalno nastavitev.

To lahko omogočimo tako, da nastavimo zastavico **enableViewCounts** na true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

To je mogoče prilagoditi brez kode na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Stran za prilagajanje gradnika z označenim potrditvenim poljem za število ogledov, tako da vsak komentar prikazuje ikono očesa in število'; title='Omogočanje števila ogledov komentarjev' app-screenshot-end]

Sledimo ID-ju uporabnika*, ki je ogledal komentar, eno teden, tako da se pri ponovnem ogledu v tem tednu število ne poveča. Če komentar ogledate po preteku tedna, se število ponovno poveča.

- *Opomba: ali anonimni ID seje, ali uporabnikov IP kot zgoščena vrednost.