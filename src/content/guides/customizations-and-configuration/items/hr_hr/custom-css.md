[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je dizajniran da se može prilagoditi. Sam widget za komentare radi unutar iframe-a iz sigurnosnih razloga, pa za primjenu
prilagođenog stiliziranja morate slijediti jedan od dva pristupa.

Prvi, najjednostavniji pristup, i onaj koji mi preferiramo, jest korištenje [stranice za prilagodbu widgeta](https://fastcomments.com/auth/my-account/customize-widget).

U stranici za prilagodbu widgeta pogledajte odjeljak "Show Advanced Options", ispod kojeg se nalazi područje označeno "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Područje za unos prilagođenog CSS-a' app-screenshot-end]

Ovaj pristup ima nekoliko prednosti:
1. Uneseni CSS se minificira prije nego se pošalje korisniku, a oblikovanje se održava konzistentnim u sučelju za uređivanje.
2. Dobivate sve prednosti sučelja za prilagodbu widgeta, na primjer lako prilagođavanje widgeta za komentare drugačije za različite stranice.
3. Kada napravimo promjene u widgetu za komentare, vaše prilagođeno stiliziranje će biti testirano kao dio našeg procesa objave.

Drugi pristup je specificirati parametar **customCSS** u konfiguraciji widgeta, kako slijedi:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Prosljeđivanje prilagođenog CSS-a'; code-example-end]

Međutim, ovo ima *ograničenja*:
1. Postoji ograničenje koliko prilagođenog CSS-a se može proslijediti prije nego što naši serveri odbiju zahtjev zbog veličine zaglavlja.
2. Morate upravljati prilagođenim CSS-om u vlastitoj infrastrukturi i sustavu za gradnju. To također može biti prednost umjesto nedostatka.
3. Postoji dodatno opterećenje slanja prilagođenog CSS-a preko mreže **dvaput** u ovom slučaju, jer ga treba poslati našim serverima, a zatim vratiti u sadržaju iframe-a. Međutim, za većinu veličina sadržaja, to nije uočljivo.
4. Uobičajena optimizacija je minificiranje CSS-a kako bi se smanjila njegova veličina na mreži, no s ovim pristupom to ćete morati sami riješiti.
5. Vaš prilagođeni CSS neće biti testiran kada mi napravimo promjene.

### Vanjske CSS datoteke

Možete instruirati widget da preuzme vanjsku datoteku koristeći `@import`!

Preporučuje se staviti `@import` u pravilo za prilagodbu. Na taj način, ako ikada budemo morali napraviti promjenu widgeta za komentare, možemo upotrijebiti našu automatizaciju
za provjeru vaše konfiguracije. Na primjer, kreirali biste pravilo za prilagodbu u sučelju za prilagodbu widgeta, kliknuli `Advanced`, i unijeli u `Custom CSS`:

    @import url(https://example.com/styles.css);

#### U kodu - nije preporučeno

Također možete učitati vanjsku CSS datoteku putem svojstva `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Vanjska CSS datoteka'; code-example-end]

Međutim, imajte na umu da vaš CSS neće biti testiran od strane nas ako to učinite. 

### Stiliziranje modalnog prozora korisničkog profila

Modalni prozori korisničkih profila također se mogu stilizirati prilagođenim CSS-om. Međutim, kako bi se osiguralo da se prilagođeno stiliziranje primijeni na korisničke profile, svi CSS selektori moraju biti prefiksirani s `.user-profile`. Bez ovog prefiksa, prilagođeno stiliziranje će biti ignorirano za modale korisničkih profila.

Na primjer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS korisničkog profila'; code-example-end]

### Povratna kompatibilnost

U FastCommentsu znamo da naši korisnici prilagođavaju widget za komentare. To je namjerno dizajnirano - zadnje što želimo je da naš proizvod uzrokuje dizajnerske nedosljednosti u vašem proizvodu.

Budući da je ovo važan dio našeg proizvoda, imamo proces izgradnje koji nam omogućava pregled promjena widgeta za komentare, za svakog korisnika, pri svakoj objavi.

Ako pronađemo manje probleme, ažurirat ćemo vaš račun kako bismo osigurali glatku objavu. Ako uočimo velike kritične promjene, to nam omogućava zaustavljanje objave.