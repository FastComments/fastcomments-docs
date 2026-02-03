FastComments vam omogoča, da zahtevate, da prvi komentatorji sprejmejo vaše pogoje storitve, preden oddajo komentar.

Ko je omogočeno:
- **Anonimni uporabniki** bodo ob vsakem komentiranju videli potrditveno polje za pogoje storitve (TOS)
- **Prijavljeni uporabniki** bodo potrditveno polje videli le pri prvem komentarju ali ko posodobite svoje pogoje storitve (TOS)

### Enabling Terms of Service

Pojdite na stran za prilagajanje vtičnika in omogočite potrditveno polje "Zahtevaj sprejem pogojev storitve":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

Privzeto potrditveno polje prikazuje "Strinjam se s Pogoji storitve in Pravilnikom o zasebnosti" z povezavami do obeh dokumentov. To besedilo lahko po potrebi prilagodite za posamezno lokalizacijo:

1. Izberite "Prilagodite besedilo po lokalizaciji"
2. Izberite lokalizacijo iz spustnega menija in vnesite svoje besedilo

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

Ko posodobite svoje pogoje storitve, nastavite datum "Zadnja posodobitev". Uporabniki, ki so pogoje sprejeli pred tem datumom, jih bodo morali ponovno sprejeti:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- Časovni žig sprejetja TOS se shranjuje za vsakega uporabnika in za vsak komentar
- Ko uporabnik sprejme TOS, je datum zabeležen v njihovem uporabniškem profilu (za posameznega najemnika)
- Če nastavite datum "Zadnja posodobitev", ki je poznejši od datuma sprejetja uporabnika, ga bo treba ponovno sprejeti
- Pri anonimnih uporabnikih, ki jih ni mogoče slediti, se potrditveno polje prikaže pri vsakem oddajanju komentarja