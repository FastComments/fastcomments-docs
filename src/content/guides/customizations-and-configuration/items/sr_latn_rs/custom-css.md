[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da bude prilagodljiv. Sam widget za komentarisanje radi unutar iframe-a iz sigurnosnih razloga, pa da primenite prilagođeni stil morate slediti jedan od dva pristupa.

Prvi, najjednostavniji pristup, i naš preferirani, je da koristite [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

Na stranici za prilagođavanje widgeta, pogledajte odeljak „Show Advanced Options“, ispod kojeg se nalazi oblast označena kao „Custom CSS“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Uređivač prilagođenog CSS-a ispod Show Advanced Options na stranici za prilagođavanje widgeta'; title='Oblast za unos prilagođenog CSS-a' app-screenshot-end]

Ovaj pristup ima neke prednosti:
1. Uneti CSS se minifikuje pre nego što se pošalje korisniku, a formatiranje se održava doslednim u UI za uređivanje.
2. Dobijate sve prednosti UI-ja za prilagođavanje widgeta, na primer lako prilagođavanje widgeta za komentarisanje različito za različite sajtove.
3. Kada napravimo promene na widgetu za komentarisanje, vaš prilagođeni stil će biti testiran kao deo našeg procesa izdavanja.

Drugi pristup je da navedete parametar **customCSS** u konfiguraciji widgeta, na sledeći način:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Prosleđivanje prilagođenog CSS-a'; code-example-end]

Međutim, ovo ima *ograničenja*:
1. Postoji ograničenje koliko prilagođenog CSS-a može biti prosleđeno pre nego što naši serveri odbiju zahtev, zbog veličine zaglavlja.
2. Morate upravljati prilagođenim CSS-om u vašoj infrastrukturi i sistemu za izgradnju. Ovo može biti i prednost, a ne samo nedostatak.
3. Postoji dodatni trošak slanja prilagođenog CSS-a preko mreže **dva puta** u ovom slučaju, jer se mora poslati našim serverima, a zatim nazad u sadržaj iframe-a. Međutim, za većinu veličina opterećenja, to nije primetno.
4. Uobičajena optimizacija je minifikacija CSS-a kako bi se smanjila njegova veličina preko mreže, ali sa ovim pristupom vi ćete to morati da uradite.
5. Vaš prilagođeni CSS neće biti testiran kada mi napravimo promene.

### Eksterni CSS fajlovi

Možete reći widgetu da preuzme eksterni fajl koristeći `@import`!

Preporučuje se da `@import` stavite u pravilo za prilagođavanje. Na ovaj način, ako ikada budemo morali da promenimo widget za komentarisanje, možemo koristiti našu automatizaciju da proverimo vaše podešavanje. Na primer, kreirali biste pravilo za prilagođavanje u UI-ju za prilagođavanje widgeta, kliknuli na `Advanced` i uneli u `Custom CSS`:

    @import url(https://example.com/styles.css);

#### U kodu – nije preporučeno

Takođe možete učitati eksterni CSS fajl putem svojstva `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Eksterni CSS fajl'; code-example-end]

Međutim, imajte na umu da vaš CSS neće moći da bude testiran od strane nas ako to uradite. 

### Stilizovanje modalnog prozora korisničkog profila

Modalni prozori korisničkog profila takođe mogu biti stilizovani prilagođenim CSS-om. Međutim, da bi se osiguralo da se prilagođeni stil primeni na korisničke profile, svi CSS selektori moraju imati prefiks `.user-profile`. Bez ovog prefiksa, prilagođeni stil će biti ignorisan za modalne prozore korisničkog profila.

Na primer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS za korisnički profil'; code-example-end]

### Unazadna kompatibilnost

U FastComments-u znamo da naši korisnici prilagođavaju widget za komentarisanje. To je po dizajnu – poslednja stvar koju želimo je da naš proizvod izazove nesklad u dizajnu vašeg proizvoda.

Pošto je ovo važan deo našeg proizvoda, imamo pipeline za izgradnju koji nam omogućava da pregledamo promene na widgetu za komentarisanje, po korisniku, pri svakom izdanju.

Ako pronađemo manje probleme, ažuriraćemo vaš nalog kako bismo osigurali da naše izdanje teče glatko. Ako uočimo velike promene koje prekidaju funkcionalnost, to nam omogućava da zaustavimo izdanje.