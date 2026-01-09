FastComments SSO (<a href="#sso">podrobnosti tukaj</a>) omogoča vašim uporabnikom komentiranje, ne da bi se morali prijaviti v drugo platformo.

Vendar to samo po sebi ne zavaruje vaših nitk komentarjev, saj so privzeto podatki komentarjev javno dostopne informacije - kdorkoli, ki si lahko ogleda stran, lahko vidi tudi komentarje.

S spremembo nastavitve lahko omejimo pridobivanje komentarjev, razen če jih zahteva skrbnik ali veljaven SSO uporabnik.

#### No-Code Setup

Ko je SSO nastavljen, lahko ogledovanje in interakcijo z našimi nitkami komentarjev preprečimo z ustvarjanjem <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravila prilagajanja</a>.

Ko to naredite, poiščite SSO in našli boste to možnost:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Omogočite jo in shranite pravilo prilagajanja.

#### Only Protect a Certain Domain or Page

Če želite zaščititi samo določen Domain ali Page, preprosto konfiguriramo pravilo prilagajanja tako, da to stori.

Na vrhu uporabniškega vmesnika za prilagajanje bomo našli dve polji, Domain and URL ID.

Če želite zaščititi samo določen domeno, v polje "domain" vnesite ustrezno domeno.

Če želite zaščititi določeno stran, v polje "URL ID" vnesite URL strani. Če imate lastno integracijo s FastComments, lahko tukaj namesto URL vnesete tudi vrsto ID-ja.

#### Security Levels

Ko zahtevate SSO, se boste morali odločiti, ali zahtevate Simple SSO ali Secure SSO. Če zahtevate Simple SSO, sta obe vrsti dovoljeni, vendar če zahtevate Secure SSO, mora biti vsebina pridobljena s Secure SSO obremenitvijo (payload), zgoščeno z vašim API key, da bo lahko prikazana.

Možnost ravni varnosti se bo pojavila, ko izberete "Require SSO To View Comments".

#### Protection Beyond Reading

Omogočitev te možnosti bo zaščitila stran ali domeno pred komentiranjem, razen če je uporabnik prijavljen preko SSO.

#### Gotchas

Uporabniki, ki so ustvarili komentarje pred vašo SSO integracijo, jih ne bodo mogli videti, razen če se prijavijo preko vaše SSO integracije.