FastComments SSO (<a href="#sso">podrobnosti tukaj</a>) svojim uporabnikom omogoča komentiranje, ne da bi se morali prijaviti v drugo platformo.

Vendar to samo ne zagotavlja varnosti vaših niti komentarjev, saj so podatki o komentarjih privzeto javno dostopni – kdorkoli, ki si lahko ogleda stran, si lahko ogleda tudi komentarje.

S spreminjanjem nastavitve lahko omejimo pridobivanje komentarjev, razen če jih zahteva skrbnik ali veljaven SSO uporabnik.

#### No-Code Setup

Ko je SSO nastavljen, lahko preprečimo ogled in interakcijo z našimi nitmi komentarjev tako, da ustvarimo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilo prilagoditve</a>.

Pri tem poiščite SSO in najdete to možnost:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Možnost Zahtevaj SSO za ogled komentarjev omogočena v pravilu prilagoditve, z izbiro ravni varnosti'; title='Zahtevaj SSO za ogled komentarjev' app-screenshot-end]

Omogočite jo in shranite pravilo prilagoditve.

#### Zaščiti le določen domen ali stran

Da zaščitimo le določen domen ali stran, preprosto nastavimo pravilo prilagoditve, da to stori.

Na vrhu uporabniškega vmesnika za prilagoditev bomo našli dva vnosa: Domen in URL ID.

Če želite zaščititi le določen domen, vnesite zadevni domen v polje "domain".

Če želite zaščititi določeno stran, vnesite URL strani v polje "URL ID". Če imate po meri integracijo s FastComments, lahko tukaj vnesete vrsto ID-ja namesto URL-ja.

#### Ravni varnosti

Ko zahtevate SSO, boste morali izbrati, ali potrebujete preprosto SSO (Simple SSO) ali varno SSO (Secure SSO). Če izberete preprosto SSO, sta dovoljeni obe možnosti, vendar če izberete varno SSO, mora biti vsebina pridobljena s varnim SSO paketom, ki je zgoščen z vašim API ključem, da jo je mogoče ogledati.

Možnost ravni varnosti se prikaže, ko izberete "Require SSO To View Comments", in začne veljati le, ko je ta možnost vklopljena.

Nastavitve <a href="#sso-simple">Samodejno preverjanje komentarjev prek preprostega SSO in dovoljenje preprostim SSO skrbnikom</a> so ločene in se uporabljajo ne glede na to, ali je ta možnost vklopljena.

#### Zaščita poleg branja

Omogočitev te možnosti bo zaščitila stran ali domen pred komentiranjem, razen če je uporabnik prijavljen prek SSO.

#### Nepričakovane težave

Katerikoli uporabniki, ki so ustvarili komentarje pred vašo SSO integracijo, jih ne bodo mogli videti, razen če se prijavijo prek vaše SSO integracije.

---