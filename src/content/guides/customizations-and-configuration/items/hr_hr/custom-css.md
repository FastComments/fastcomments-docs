[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da se prilagođava. Sam widget za komentiranje radi unutar iframea iz sigurnosnih razloga, pa za primjenu prilagođenog stiliziranja morate slijediti jedan od dva pristupa.

Prvi, najjednostavniji pristup, i naš preferirani, je korištenje stranice za prilagodbu widgeta.

Na stranici za prilagodbu widgeta, pogledajte odjeljak „Show Advanced Options“, ispod kojeg se nalazi područje označeno „Custom CSS“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Uređivač prilagođenog CSS-a pod Opcijama naprednog prikaza na stranici prilagodbe widgeta'; title='Područje unosa prilagođenog CSS-a' app-screenshot-end]

Ovaj pristup ima neke prednosti:
1. Uneseni CSS se minificira prije nego što se pošalje korisniku, a formatiranje ostaje dosljedno u sučelju za uređivanje.
2. Dobivate sve prednosti UI-a za prilagodbu widgeta, na primjer lako prilagođavanje widgeta za komentiranje različito za različite web‑stranice.
3. Kada napravimo promjene na widgetu za komentiranje, vaše prilagođeno stiliziranje bit će testirano kao dio našeg procesa izdanja.

Drugi pristup je specificirati parametar **customCSS** u konfiguraciji widgeta, na sljedeći način:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Prosljeđivanje prilagođenog CSS-a'; code-example-end]

Međutim, ovo ima *ograničenja*:
1. Postoji ograničenje koliko prilagođenog CSS-a se može proslijediti prije nego što naši serveri odbiju zahtjev, zbog veličine zaglavlja.
2. Morate upravljati prilagođenim CSS-om u svojoj infrastrukturi i sustavu izgradnje. To može biti i prednost, a ne samo nedostatak.
3. Postoji dodatni trošak slanja prilagođenog CSS-a preko mreže **dvaput** u ovom slučaju, jer se mora poslati našim serverima, a zatim natrag u sadržaj iframea. Međutim za većinu veličina opterećenja to nije primjetno.
4. Uobičajena optimizacija je minificiranje CSS-a kako bi se smanjila njegova veličina na mreži, ali s ovim pristupom to ćete morati sami upravljati.
5. Vaš prilagođeni CSS neće biti testiran kada mi napravimo promjene.

### Vanjske CSS datoteke

Možete reći widgetu da preuzme vanjsku datoteku koristeći `@import`!

Preporučuje se staviti `@import` u pravilo prilagodbe. Na taj način, ako ikada trebamo napraviti promjenu na widgetu za komentiranje, možemo koristiti našu automatizaciju za provjeru vaše postavke. Na primjer, kreirali biste pravilo prilagodbe u UI‑u za prilagodbu widgeta, kliknuli `Advanced` i unijeli u `Custom CSS`:

    @import url(https://example.com/styles.css);

#### U kodu – nije preporučeno

Također možete učitati vanjsku CSS datoteku putem svojstva `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Vanjska CSS datoteka'; code-example-end]

Međutim, imajte na umu da vaš CSS neće moći biti testiran od strane nas ako to učinite. 

### Stiliziranje modalnog prozora korisničkog profila

Modalni prozori korisničkog profila također se mogu stilizirati prilagođenim CSS-om. Međutim, kako bi se osiguralo da se prilagođeni stil primijeni na korisničke profile, svi CSS selektori moraju imati prefiks `.user-profile`. Bez tog prefiksa, prilagođeni stil će biti zanemaren za modalne prozore korisničkog profila.

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS korisničkog profila'; code-example-end]

### Unazadna kompatibilnost

U FastCommentsu znamo da naši kupci prilagođavaju widget za komentiranje. To je namjerno – posljednje što želimo je da naš proizvod uzrokuje nesklad u dizajnu vašeg proizvoda.

Budući da je to važan dio našeg proizvoda, imamo proces izgradnje koji nam omogućuje pregled promjena widgeta za komentiranje po kupcu, pri svakom izdanju.

Ako pronađemo manje probleme, ažurirat ćemo vaš račun kako bismo osigurali da naše izdanje teče glatko. Ako vidimo veće promjene koje prekidaju funkcionalnost, to nam omogućuje da zaustavimo izdanje.