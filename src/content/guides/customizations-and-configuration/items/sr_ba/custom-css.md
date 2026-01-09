---
[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da bude prilagodljiv. Sam widget za komentare radi unutar iframe-a iz sigurnosnih razloga, pa da biste primijenili prilagođeni stil morate slijediti jedan od dva pristupa.

Prvi, najlakši pristup, i onaj koji preporučujemo, je da koristite [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

Na stranici za prilagođavanje widgeta, pogledajte odjeljak "Show Advanced Options", ispod kojeg se nalazi oblast označena kao "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Ovaj pristup ima neke prednosti:
1. Uneseni CSS se minificira prije nego što se pošalje korisniku, a formatiranje se održava konzistentnim u uređivačkom korisničkom interfejsu.
2. Dobijate sve prednosti korisničkog interfejsa za prilagođavanje widgeta, na primjer lako prilagođavanje widgeta za komentare različito za različite sajtove.
3. Kada napravimo promjene na widgetu za komentare, vaš prilagođeni stil će biti testiran kao dio našeg procesa izdavanja.

Drugi pristup je da u konfiguraciji widgeta navedete parametar **customCSS**, na sljedeći način:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Međutim, ovo ima *ograničenja*:
1. Postoji ograničenje koliko custom CSS-a može biti proslijeđeno prije nego što naši serveri odbace zahtjev, zbog veličine headers.
2. Morate upravljati custom CSS-om u vašoj infrastrukturi i build sistemu. Ovo također može biti prednost, a ne nedostatak.
3. Postoji dodatni overhead slanja custom CSS-a preko mreže **dvaput** u ovom slučaju, jer mora biti poslan na naše servere, a zatim vraćen u sadržaju iframe-a. Međutim, za većinu veličina payloada, ovo nije primjetno.
4. Uobičajena optimizacija je minificiranje CSS-a kako bi se smanjila njegova veličina preko mreže, međutim s ovim pristupom vi ćete to morati sami obraditi.
5. Vaš custom CSS neće biti testiran kada mi napravimo promjene.

### Eksterni CSS fajlovi

Možete naložiti widgetu da povuče eksterni fajl koristeći `@import`!

Preporučeno je staviti `@import` u pravilo za prilagođavanje. Na ovaj način, ako ikada budemo morali napraviti promjenu na widgetu za komentare, možemo koristiti naše alate za automatizaciju da provjerimo vašu konfiguraciju. Dakle, na primjer, kreirali biste pravilo za prilagođavanje u korisničkom interfejsu za prilagođavanje widgeta, kliknuli `Advanced`, i unijeli u `Custom CSS`:

    @import url(https://example.com/styles.css);

#### U kodu - nije preporučeno

Također možete učitati eksterni CSS fajl putem svojstva `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Međutim, imajte na umu da vaš CSS neće moći biti testiran od strane nas ako ovo uradite. 

### Stilizovanje modal prozora korisničkog profila

Modal prozori korisničkih profila također se mogu stilizovati pomoću custom CSS-a. Međutim, kako bi se osiguralo da se prilagođeni stil primijeni na korisničke profile, svi CSS selektori moraju biti prefiksirani sa `.user-profile`. Bez ovog prefiksa, prilagođeni stil će biti zanemaren za modal prozore korisničkih profila.

Na primjer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Kompatibilnost unazad

U FastComments-u, znamo da naši korisnici prilagođavaju widget za komentare. To je namjerno - posljednje što želimo je da naš proizvod izazove neslaganja u dizajnu vašeg proizvoda.

Pošto je ovo važan dio našeg proizvoda, imamo build pipeline koji nam omogućava da pregledamo promjene na widgetu za komentare, po korisniku, pri svakom izdanju.

Ako uočimo manje probleme, ažurirat ćemo vaš račun kako bi naše izdanje prošlo glatko. Ako uočimo velike promjene koje bi prekinule funkcionalnost, to nam omogućava da zaustavimo izdanje.

---