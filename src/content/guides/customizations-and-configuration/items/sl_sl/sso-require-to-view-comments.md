---
FastComments SSO (<a href="#sso">podrobnosti tukaj</a>) svojim uporabnikom omogoča način komentiranja, ne da bi se morali prijaviti v drugo platformo.

Vendar to samo ne zagotavlja varnosti vaših niti komentarjev, saj so podatki o komentarjih privzeto javno dostopni – kdorkoli, ki lahko vidi stran, lahko vidi tudi komentarje.

S spreminjanjem nastavitve lahko omejimo pridobivanje komentarjev, razen če to stori skrbnik ali veljaven SSO uporabnik.

#### Nastavitev brez kode

Ko je SSO nastavljen, lahko preprečimo ogled in interakcijo z našimi nitmi komentarjev, tako da ustvarimo <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilo prilagoditve</a>.

Pri tem poiščite SSO in najdete to možnost:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Možnost Zahtevaj SSO za ogled komentarjev omogočena v pravilu prilagoditve, z izbiro varnostne ravni'; title='Zahtevaj SSO za ogled komentarjev' app-screenshot-end]

Omogočite jo in shranite pravilo prilagoditve.

#### Zaščiti le določen domen ali stran

Če želite zaščititi le določen domen ali stran, preprosto nastavimo pravilo prilagoditve, da to omogoča.

Na vrhu uporabniškega vmesnika za prilagoditev bomo našli dva vnosa, Domen in URL ID.

Če želite zaščititi le določen domen, vnesite ta domen v polje "domain".

Če želite zaščititi določeno stran, vnesite URL strani v polje "URL ID". Če imate po meri integracijo s FastComments, lahko tukaj vnesete vrsto ID-ja namesto URL-ja.

#### Varnostne ravni

Ko zahtevate SSO, boste morali izbrati, ali potrebujete preprosto SSO ali varno SSO. Če izberete preprosto SSO, sta dovoljeni obe možnosti, vendar če izberete varno SSO, mora biti vsebina pridobljena z varnim SSO paketom, ki je zgoščen z vašim API ključem, da jo je mogoče ogledati.

Možnost varnostne ravni se bo pojavila, ko izberete "Zahtevaj SSO za ogled komentarjev".

#### Zaščita poleg branja

Omogočanje te možnosti bo zaščitilo stran ali domen pred komentiranjem, razen če je uporabnik prijavljen prek SSO.

#### Morebitne težave

Uporabniki, ki so ustvarili komentarje pred vašo SSO integracijo, jih ne bodo mogli videti, razen če se prijavijo prek vaše SSO integracije.

---