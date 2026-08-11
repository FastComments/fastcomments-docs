[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je zasnovan tako, da ga je mogoče prilagoditi. Sam pripomoček za komentiranje teče v iframeu iz varnostnih razlogov, zato morate za uporabo po meri oblikovanja slediti enemu od dveh pristopov.

Prvi, najlažji pristop, ki ga mi priporočamo, je uporaba [strani za prilagajanje gradnika](https://fastcomments.com/auth/my-account/customize-widget).

Na strani za prilagajanje gradnika poiščite razdelek "Prikaži napredne možnosti", pod katerim je območje označeno kot "Po meri CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Urejevalnik po meri CSS pod Prikaži napredne možnosti na strani za prilagajanje gradnika'; title='Območje vnosa po meri CSS' app-screenshot-end]

Ta pristop ima nekaj prednosti:
1. Vneseni CSS je minificiran, preden je poslan uporabniku, oblikovanje pa ostane dosledno v uporabniškem vmesniku za urejanje.
2. Dobite vse prednosti uporabniškega vmesnika za prilagajanje gradnika, na primer enostavno prilagajanje pripomočka za komentiranje za različna spletna mesta.
3. Ko naredimo spremembe v pripomočku za komentiranje, bo vaše po meri oblikovanje preizkušeno kot del našega postopka izdajanja.

Drugi pristop je, da v konfiguraciji gradnika določite parameter **customCSS**, kot sledi:

[code-example-start config = {customCSS: "button { background: red; }"}; linesToHighlight = [6]; title = 'Posredovanje po meri CSS'; code-example-end]

Vendar ima to *omejitve*:
1. Obstaja omejitev, koliko po meri CSS lahko pošljemo, preden naši strežniki zavrnejo zahtevo, zaradi velikosti glave.
2. Morate upravljati po meri CSS v svoji infrastrukturi in sistemu gradnje. To je lahko tudi prednost, ne le slabost.
3. V tem primeru je dodatna obremenitev pošiljanja po meri CSS po omrežju **dvakrat**, saj mora biti poslan na naše strežnike in nato nazaj v vsebino iframea. Vendar pa za večino velikosti paketov to ni opazno.
4. Pogosta optimizacija je minifikacija CSS, da se zmanjša njegova velikost po omrežju, vendar boste pri tem pristopu to morali sami upravljati.
5. Vaš po meri CSS ne bo preizkušen, ko bomo naredili spremembe.

### External CSS Files

Gradniku lahko poveste, naj naloži zunanjo datoteko z uporabo `@import`!

Priporočamo, da `@import` postavite v pravilo prilagajanja. Na ta način, če bomo kdaj morali spremeniti pripomoček za komentiranje, lahko uporabimo naše avtomatizacijske orodje za preverjanje nastavitve. Na primer, ustvarite pravilo prilagajanja v uporabniškem vmesniku za prilagajanje gradnika, kliknete `Advanced` in vnesete v `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

Zunanjo CSS datoteko lahko naložite tudi prek lastnosti `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);"}; linesToHighlight = [6]; title = 'Zunanja CSS datoteka'; code-example-end]

Vendar pa se spomnite, da vaš CSS ne bo mogel biti preizkušen s strani nas, če to storite. 

### User Profile Modal Styling

Modalna okna uporabniškega profila je mogoče tudi stilizirati s po meri CSS. Vendar pa, da zagotovite, da se po meri oblikovanje uporabi na uporabniških profilih, morajo biti vsi CSS selektorji predpripeti z `.user-profile`. Brez tega predpone bo po meri oblikovanje prezrto za modalna okna uporabniškega profila.

Na primer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }"}; title = 'CSS uporabniškega profila'; code-example-end]

### Backwards Compatibility

Pri FastComments vemo, da naši uporabniki prilagajajo pripomoček za komentiranje. To je po zasnovi – zadnja stvar, ki jo želimo, je, da bi naš izdelek povzročil neskladnosti v oblikovanju vašega izdelka.

Ker je to pomemben del našega izdelka, imamo gradbeni proces, ki nam omogoča pregled sprememb pripomočka za komentiranje po strankah pri vsaki izdaji.

Če najdemo manjše težave, bomo posodobili vaš račun, da zagotovimo nemoteno izdajo. Če opazimo večje prelomne spremembe, nam to omogoča, da izdajo ustavimo.