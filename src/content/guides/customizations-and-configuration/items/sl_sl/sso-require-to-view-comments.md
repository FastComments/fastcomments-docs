FastComments SSO (<a href="#sso">podrobnosti tukaj</a>) vašim uporabnikom omogoča komentiranje brez prijave na drugo platformo.

Vendar to samo po sebi ne zavaruje vaših komentarjev, saj so podatki o komentarjih privzeto javno dostopne informacije - vsakdo, ki si lahko ogleda stran, si lahko ogleda tudi komentarje.

S spremembo nastavitve lahko omejimo pridobivanje komentarjev, razen za administratorje ali veljavne SSO uporabnike.

#### Nastavitev brez kode

Ko je SSO nastavljen, lahko preprečimo ogled in interakcijo z našimi komentarji z ustvarjanjem <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravila prilagoditve</a>.

Pri tem poiščite SSO in našli boste to možnost:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Možnost zahtevaj SSO za ogled komentarjev omogočena v pravilu prilagoditve, z izbiro varnostne ravni'; title='Zahtevaj SSO za ogled komentarjev' app-screenshot-end]

Omogočite jo in shranite pravilo prilagoditve.

#### Zaščita samo določene domene ali strani

Za zaščito samo določene domene ali strani preprosto nastavimo pravilo prilagoditve.

Na vrhu vmesnika za prilagoditve bomo našli dve polji: Domain in URL ID.

Za zaščito določene domene vnesite dotično domeno v polje "domain".

Za zaščito določene strani vnesite URL strani v polje "URL ID". Če imate prilagojeno integracijo s FastComments, lahko tukaj vnesete vrsto ID-ja namesto URL-ja.

#### Varnostne ravni

Ko zahtevate SSO, se boste morali odločiti, ali zahtevate Simple SSO ali Secure SSO. Če zahtevate Simple SSO, sta dovoljena oba, če pa zahtevate Secure SSO, mora biti vsebina pridobljena z Secure SSO payloadom, ki je hashiran z vašim API ključem, da si jo je mogoče ogledati.

Možnost varnostne ravni se prikaže, ko izberete "Zahtevaj SSO za ogled komentarjev", in deluje samo, dokler je ta možnost vklopljena.

Nastavitvi <a href="#sso-simple">Samodena potrditev komentarjev prek Simple SSO in Dovoli Simple SSO administratorje</a> sta ločeni in veljata ne glede na to, ali je ta možnost vklopljena.

#### Zaščita onkraj branja

Omogočitev te možnosti bo zavarovala stran ali domeno pred komentiranjem, razen če je uporabnik prijavljen prek SSO.

#### Pasti

Uporabniki, ki so ustvarili komentarje pred vašo SSO integracijo, jih ne bodo mogli videti, razen če se prijavijo prek vaše SSO integracije.