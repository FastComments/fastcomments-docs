[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je zasnovan za prilagajanje. Samo pripomoček za komentarje se iz varnostnih razlogov izvaja znotraj iframe-a, zato morate za uporabo
sloga po meri slediti enemu od dveh pristopov.

Prvi, najlažji pristop, ki ga tudi mi priporočamo, je uporaba [strani za prilagajanje pripomočka](https://fastcomments.com/auth/my-account/customize-widget).

Na strani za prilagajanje pripomočka poglejte razdelek "Show Advanced Options", pod katerim je območje z oznako "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Urejevalnik CSS po meri pod Show Advanced Options na strani za prilagajanje pripomočka'; title='Vnosno območje za CSS po meri' app-screenshot-end]

Ta pristop ima nekaj prednosti:
1. Vneseni CSS je pred pošiljanjem uporabniku pomanjšan (minificiran), oblika pa je v urejevalnem vmesniku ohranjena dosledna.
2. Izkoristite vse prednosti vmesnika za prilagajanje pripomočka, na primer enostavno prilagajanje pripomočka za komentarje različno za različne spletne strani.
3. Ko naredimo spremembe v pripomočku za komentarje, bo vaš slog po meri testiran kot del našega procesa izdaje.

Drugi pristop je določitev parametra **customCSS** v konfiguraciji pripomočka, takole:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Posredovanje CSS po meri'; code-example-end]

Vendar ima to *omejitve*:
1. Obstaja omejitev, koliko CSS po meri lahko posredujemo, preden naši strežniki zavrnejo zahtevo, zaradi velikosti glav (headers).
2. CSS po meri morate upravljati v svoji infrastrukturi in sistemu za gradnjo. To je lahko tudi prednost, ne slabost.
3. V tem primeru je dodatna obremenitev, ker se CSS po meri pošilja po omrežju **dvakrat**, saj ga je treba poslati našim strežnikom in nato nazaj v vsebino iframe-a. Za večino velikosti podatkov pa tega ni opaziti.
4. Pogosta optimizacija je pomanjšanje (minificiranje) CSS za zmanjšanje velikosti prenosov, vendar pri tem pristopu morate to urediti sami.
5. Vaš CSS po meri ne bo testiran, ko naredimo spremembe.

### Zunanje datoteke CSS

Pripomočku lahko naročite, naj pridobi zunanjo datoteko z uporabo `@import`!

Priporočamo, da `@import` umestite v pravilo prilagajanja. Tako, če bomo kdaj morali narediti spremembo v pripomočku za komentarje, lahko z našo avtomatizacijo
preverimo vašo nastavitev. Torej bi na primer ustvarili pravilo prilagajanja v vmesniku Widget Customization, kliknili `Advanced` in vnesli v `Custom CSS`:

    @import url(https://example.com/styles.css);

#### V kodi - ni priporočljivo

Zunanjo datoteko CSS lahko naložite tudi prek lastnosti `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Zunanja datoteka CSS'; code-example-end]

Vendar ne pozabite, da vašega CSS v tem primeru ne bomo mogli testirati.

### Slog modala profila uporabnika

Tudi modale profilov uporabnikov je mogoče oblikovati s CSS po meri. Vendar, da se slog po meri uporabi na profilih uporabnikov, morajo biti vsi CSS selektorji predponjeni z `.user-profile`. Brez te predpone bo slog po meri pri modalih profilov uporabnikov prezrt.

Na primer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS profila uporabnika'; code-example-end]

### Združljivost z nazaj

Pri FastComments vemo, da naši stranke prilagajajo pripomoček za komentarje. To je po zasnovi - zadnja stvar, ki si je želimo, je, da bi naš izdelek povzročil
nedoslednosti v oblikovanju vašega izdelka.

Ker je to pomemben del našega izdelka, imamo gradbeni proces (build pipeline), ki nam omogoča pregled sprememb pripomočka za komentarje, za vsako stranko, ob vsaki izdaji.

Če odkrijemo manjše težave, bomo posodobili vaš račun, da bo izdaja potekala gladko. Če opazimo večje spremembe, ki bi povzročile težave, nam to omogoča, da izdajo ustavimo.