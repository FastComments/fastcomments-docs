[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da bude prilagodljiv. Sam widget za komentarisanje radi unutar iframe‑a iz sigurnosnih razloga, pa da primenite prilagođeni stil morate slediti jedan od dva pristupa.

Prvi, najlakši pristup, i onaj koji mi preporučujemo, je da koristite [stranicu za prilagođavanje widgeta](https://fastcomments.com/auth/my-account/customize-widget).

Na stranici za prilagođavanje widgeta, pogledajte odeljak „Show Advanced Options“, ispod kojeg se nalazi oblast označena „Custom CSS“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Uređivač prilagođenog CSS-a ispod Opcije za napredne postavke na stranici za prilagođavanje widgeta'; title='Oblast unosa prilagođenog CSS-a' app-screenshot-end]

Ovaj pristup ima neke prednosti:
1. Uneti CSS se minifikuje pre slanja korisniku, a formatiranje se održava doslednim u UI za uređivanje.
2. Dobijate sve prednosti UI‑a za prilagođavanje widgeta, na primer lako prilagođavanje widgeta za komentarisanje različito za različite sajtove.
3. Kada napravimo promene na widgetu za komentarisanje, vaš prilagođeni stil će biti testiran kao deo našeg procesa izdavanja.

Drugi pristup je da navedete parametar **customCSS** u konfiguraciji widgeta, na sledeći način:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Međutim, ovo ima *ograničenja*:
1. Postoji ograničenje koliko prilagođenog CSS‑a može biti prosleđeno pre nego što naši serveri odbiju zahtev, zbog veličine zaglavlja.
2. Morate upravljati prilagođenim CSS‑om u vašoj infrastrukturi i sistemu za izgradnju. Ovo može biti i prednost, a ne samo nedostatak.
3. Postoji dodatni trošak slanja prilagođenog CSS‑a preko mreže **dva puta** u ovom slučaju, jer se mora poslati našim serverima, a zatim nazad u sadržaj iframe‑a. Međutim, za većinu veličina opterećenja, to nije primetno.
4. Uobičajena optimizacija je minifikacija CSS‑a kako bi se smanjila njegova veličina preko mreže, ali sa ovim pristupom vi ćete to morati da uradite.
5. Vaš prilagođeni CSS neće biti testiran kada mi napravimo promene.

### External CSS Files

Možete reći widgetu da preuzme eksterni fajl koristeći `@import`!

Preporučuje se da `@import` stavite u pravilo prilagođavanja. Na taj način, ako ikada budemo morali da promenimo widget za komentarisanje, možemo koristiti našu automatizaciju da proverimo vaše podešavanje. Na primer, kreirali biste pravilo prilagođavanja u UI‑u za prilagođavanje widgeta, kliknuli na `Advanced` i uneli u `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

Možete takođe učitati eksterni CSS fajl putem svojstva `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Međutim, imajte na umu da vaš CSS neće moći da bude testiran od strane nas ako to uradite.

### User Profile Modal Styling

Modalni prozori korisničkih profila takođe mogu biti stilizovani prilagođenim CSS‑om. Međutim, da bi se osiguralo da se prilagođeni stil primeni na korisničke profile, svi CSS selektori moraju biti prefiksirani sa `.user-profile`. Bez ovog prefiksa, prilagođeni stil će biti ignorisan za modalne prozore korisničkih profila.

Na primer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

U FastComments‑u znamo da naši korisnici prilagođavaju widget za komentarisanje. To je po dizajnu – poslednja stvar koju želimo je da naš proizvod izazove nedoslednosti u dizajnu vašeg proizvoda.

Pošto je ovo važan deo našeg proizvoda, imamo pipeline za izgradnju koji nam omogućava da pregledamo promene widgeta za komentarisanje, po korisniku, pri svakom izdanju.

Ako pronađemo manje probleme, ažuriraćemo vaš nalog kako bismo osigurali da izdanje prođe glatko. Ako uočimo veće, razarajuće promene, to nam omogućava da zaustavimo izdanje.