[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments je zasnovan za prilagajanje. Komentarni pripomoček sam teče znotraj iframe-a iz varnostnih razlogov, zato morate za uporabo
lastnega oblikovanja slediti eni od dveh pristopov.

Prvi, najlažji pristop, in tisti, ki ga priporočamo, je uporaba [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

Na strani za prilagajanje widgeta si oglejte razdelek "Show Advanced Options", pod katerim je območje z oznako "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Ta pristop ima nekatere prednosti:
1. Vneseni CSS se zmanjša (minificira), preden ga pošljemo uporabniku, in oblikovanje ostane dosledno v urejevalnem vmesniku.
2. Dobite vse prednosti uporabniškega vmesnika za prilagajanje widgeta, na primer enostavno različne prilagoditve komentarskega widgeta za različna spletna mesta.
3. Ko naredimo spremembe v komentarni widget, bo vaše lastno oblikovanje testirano kot del našega procesa izdaje.

Drugi pristop je določitev parametra **customCSS** v konfiguraciji widgeta, kot sledi:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Vendar pa ima to *omejitve*:
1. Obstaja omejitev, koliko lastnega CSS-a je mogoče poslati, preden naši strežniki zavrnejo zahtevo zaradi velikosti header-jev.
2. Morate upravljati lastni CSS v svoji infrastrukturi in sistemu za gradnjo. To je lahko tudi prednost namesto slabosti.
3. Pri tem primeru obstaja dodatna obremenitev pošiljanja lastnega CSS-a prek omrežja **dvakrat**, saj ga je treba poslati našim strežnikom in ga nato poslati nazaj v vsebini iframe-a. Vendar pri večini velikosti paketov to ni opazno.
4. Pogosta optimizacija je minificiranje CSS-a za zmanjšanje njegove velikosti na omrežju, vendar boste to pri tem pristopu morali urejati sami.
5. Vašega lastnega CSS-a ne bomo testirali, čeprav ga uporabljate na ta način.

### Zunanje datoteke CSS

Lahko naročite widget, naj pridobi zunanjo datoteko z uporabo `@import`!

Priporočljivo je, da `@import` postavite v pravilo za prilagajanje. Tako, če bomo kdaj morali spremeniti komentarni widget, lahko uporabimo naše avtomatizirane
orodja za preverjanje vaše nastavitve. Torej na primer bi ustvarili pravilo za prilagajanje v vmesniku za prilagajanje widgeta, kliknili `Advanced` in vnesli v `Custom CSS`:

    @import url(https://example.com/styles.css);

#### V kodi - ni priporočljivo

Zunanjo datoteko CSS lahko naložite tudi preko lastnosti `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Vendar ne pozabite, da vašega CSS-a v tem primeru ne bomo mogli testirati.

### Stiliranje pojavnega okna profila uporabnika

Pojavna okna profilov uporabnikov je mogoče prav tako stilirati z lastnim CSS-om. Vendar pa, da zagotovite, da se lastno oblikovanje uporabi za profile uporabnikov, morajo biti vsi CSS selektorji predponjeni z `.user-profile`. Brez te predpone bo lastno oblikovanje za pojavna okna profila uporabnika prezrto.

Na primer:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Združljivost z prejšnjimi različicami

V FastComments vemo, da naši kupci prilagajajo komentarni widget. To je načrtno - zadnje, kar želimo, je, da naš izdelek povzroča oblikovne
neskladnosti v vašem izdelku.

Ker je to pomemben del našega izdelka, imamo proces gradnje, ki nam omogoča pregled sprememb komentarnega widgeta, za vsakega kupca, pri vsaki izdaji.

Če najdemo manjše težave, bomo posodobili vaš račun, da zagotovimo nemoteno izdajo. Če opazimo večje uničujoče spremembe, nam to omogoča zaustavitev izdaje.

---