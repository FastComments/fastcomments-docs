---
[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Ko prikazujemo nit komentarjev ali oddajamo komentar, FastComments mora vedeti, na kateri strani, članku ali izdelku ti komentarji pripadajo.

Za to uporabljamo nekaj, čemur pravimo "URL ID". To je lahko identifikator, na primer niz ali število, ali URL.

Privzeto, če ne določite urlId, bo to postala URL strani. Vzamemo trenutni URL strani in ga očistimo, da odstranimo morebitne pogoste marketinške parametre ali identifikatorje za sledenje.

V primeru integracij tretjih oseb, kot je WordPress, bo naš vtičnik običajno uporabil identifikator, ki predstavlja trenutno ogledano vsebino kot URL ID, na primer ID članka/strani.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Ena od stvari, na katero se bomo pogosto sklicevali v tem dokumentu, je <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Vmesnik za prilagajanje pripomočka</a>.

Ta vmesnik lahko uporabite za številne spremembe komentarnega pripomočka brez uporabe kode.

Pri ustvarjanju pravila za prilagajanje bomo pogosto želeli, da velja za vse strani na naši strani. Vendar pa v nekaterih primerih želimo prilagoditi komentarni pripomoček na določeni strani, bodisi za uporabo posebnega oblikovanja ali morda, da so komentarji za to stran anonimni. Na primer, lahko na nekaterih straneh prikažete komentarje v živo takoj, medtem ko jih na drugih skrijete za gumbi za obvestila.

Vse to je mogoče preko polja za vnos URL ID na tej strani, ki izgleda takole:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

Vrednost v tem polju naj se ujema s parametrom *urlId*, ki ga posredujete komentarni pripomoček. Če želite, da je vaše pravilo za prilagajanje neodvisno od *urlId*, pustite to polje prazno ali vnesite *.

Od leta 2023 polje `URL ID` v prilagajanju pripomočka zdaj podpira tudi vzorce! Na primer, lahko imate `*/blog/*` za dodajanje slogov, specifičnih za vaš blog, in `*/store/*` za sloge specifične za vašo trgovino, vse to ob uporabi iste domene.

### Pasti

1. Če ima vaša stran hash parametre (na primer example.com#page-1) - to bo privzeto postalo del URL ID.
2. Med migracijami, na primer iz WordPress v Gatsby, boste morda morali po začetni migraciji migrirati vrednosti komentarjev, povezanih z URL ID. Za to nas kontaktirajte.

---