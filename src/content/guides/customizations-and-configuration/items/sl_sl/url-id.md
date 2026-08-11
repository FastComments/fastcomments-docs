[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Pri izrisu niza komentarjev ali pri pisanju komentarja FastComments mora vedeti, kateri strani, članek ali izdelek ti komentarji pripadajo.

Za to uporabljamo nekaj, kar imenujemo "URL ID". To je lahko identifikator, kot je niz ali številka, ali pa URL.

Privzeto, če ne določite urlId, bo to URL strani. Vzeli bomo trenutni URL strani in ga očistili, da odstranimo morebitne običajne marketinške parametre ali sledilne identifikatorje.

V primeru integracij tretjih strani, kot je WordPress, naš vtičnik običajno uporabi identifikator, ki predstavlja trenutno prikazano informacijo, kot URL ID, na primer ID članka/strani.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Določanje po meri URL ID'; code-example-end]

Ena stvar, na katero bomo v tem dokumentu pogosto sklicovali, je <a href="https://fastcomments.com/auth/my-account/customize-widget/new">uporabniški vmesnik za prilagajanje gradnika</a>.

Ta vmesnik se lahko uporabi za številne spremembe gradnika za komentarje brez uporabe kode.

Ko ustvarjamo pravilo za prilagajanje, ga pogosto želimo uporabiti za vse strani našega spletnega mesta. Vendar pa v nekaterih primerih želimo prilagoditi gradnik za komentarje na določeni strani, bodisi za uporabo po meri oblikovanja, bodisi za anonimnost komentarjev na tej strani. Lahko tudi, na primer, omogočite, da se živi komentarji takoj prikažejo na nekaterih straneh, medtem ko so na drugih skriti pod obvestilnimi gumbi.

Vse to je mogoče preko vnosnega polja URL ID na tej strani, ki izgleda takole:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Polje URL ID, uporabljeno za omejitev pravila prilagajanja na eno stran ali na vzorec, kot je */blog/*'; title='Vnos URL ID na strani za prilagajanje gradnika' app-screenshot-end]

Vrednost v tem polju mora ustrezati parametru *urlId*, ki se posreduje gradniku za komentarje. Če želite, da je vaše pravilo prilagajanja neodvisno od *urlId*, pustite to polje prazno ali vnesite *.

Od leta 2023 polje `URL ID` v prilagajanju gradnika zdaj sprejema tudi vzorce! Na primer, lahko imate `*/blog/*` za dodajanje oblikovanja, specifičnega za vaš blog, in `*/store/*` za oblikovanje, specifično za vašo trgovino, vse pri uporabi iste domene.

### Pastrežki

1. Če vaša stran vsebuje hash parametre (npr. example.com#page-1) – ti bodo privzeto postali del URL ID.
2. Med migracijami, na primer iz WordPressa v Gatsby, boste morda morali po začetni migraciji prenesti vrednosti komentarjev URL ID. Za to nas kontaktirajte.

---