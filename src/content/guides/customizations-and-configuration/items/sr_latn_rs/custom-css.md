[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da bude prilagodljiv. Sam widget za komentare se izvršava unutar iframe-a iz bezbednosnih razloga, pa da biste primenili prilagođeni stil morate slediti jedan od dva pristupa.

Prvi, najlakši pristup, i onaj koji mi preferiramo, je da koristite [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

U stranici za prilagođavanje widgeta, pogledajte odeljak "Show Advanced Options", ispod kojeg se nalazi oblast označena kao "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Ovaj pristup ima neke prednosti:
1. Uneseni CSS se minifikuje pre nego što se pošalje korisniku, a formatiranje se održava konzistentnim u UI za uređivanje.
2. Dobijate sve prednosti UI za prilagođavanje widgeta, na primer jednostavno prilagođavanje widgeta za komentare različitim sajtovima.
3. Kada napravimo promene u widgetu za komentare, vaš prilagođeni stil biće testiran kao deo našeg procesa objavljivanja.

Drugi pristup je da navedete **customCSS** parametar u konfiguraciji widgeta, na sledeći način:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Međutim, ovo ima *ograničenja*:
1. Postoji ograničenje koliko prilagođenog CSS-a može biti poslato pre nego što naši serveri odbace zahtev, zbog veličine zaglavlja.
2. Morate upravljati prilagođenim CSS-om u svojoj infrastrukturi i build sistemu. Ovo može biti i prednost umesto mane.
3. Postoji dodatni overhead slanja prilagođenog CSS-a preko mreže **dvaput** u ovom slučaju, jer se mora poslati našim serverima, a zatim vratiti u sadržaju iframe-a. Međutim, za većinu veličina payload-a ovo nije primetno.
4. Uobičajena optimizacija je minifikacija CSS-a da bi se smanjila veličina prenosa, međutim sa ovim pristupom ćete to morati sami da rešite.
5. Vaš prilagođeni CSS neće biti testiran kada napravimo promene.

### External CSS Files

Možete reći widgetu da preuzme eksterni fajl koristeći `@import`!

Preporučuje se da stavite `@import` u pravilo za prilagođavanje. Na taj način, ako ikada budemo morali da napravimo promenu u widgetu za komentare, možemo koristiti naše automatizovane alate da verifikujemo vašu konfiguraciju. Dakle, na primer, kreirali biste pravilo za prilagođavanje u Widget Customization UI, kliknuli `Advanced`, i uneli u `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

Takođe možete učitati eksterni CSS fajl putem `customCSS` svojstva:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Međutim, zapamtite da vaš CSS neće moći biti testiran od strane nas ako ovo uradite. 

### User Profile Modal Styling

Modal prozori korisničkog profila takođe se mogu stilizovati prilagođenim CSS-om. Međutim, da bi se osiguralo da se prilagođeni stil primeni na korisničke profile, svi CSS selektori moraju biti prefiksirani sa `.user-profile`. Bez ovog prefiksa, prilagođeni stil biće ignorisan za modal prozore korisničkog profila.

Na primer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

U FastComments-u znamo da naši korisnici prilagođavaju widget za komentare. To je namerno - poslednje što želimo je da naš proizvod izazove dizajnerske neusaglašenosti u vašem proizvodu.

Pošto je ovo važan deo našeg proizvoda, imamo build pipeline koji nam omogućava da pregledamo promene u widgetu za komentare po korisniku, za svako objavljivanje.

Ako pronađemo manje probleme, ažuriraćemo vaš nalog kako bismo osigurali da objavljivanje prođe glatko. Ako uočimo ozbiljne prekidajuće promene, ovo nam omogućava da zaustavimo objavljivanje.

---