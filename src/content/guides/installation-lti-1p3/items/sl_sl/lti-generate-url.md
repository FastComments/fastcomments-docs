#### Pomaknite se na konfiguracijo LTI 1.3

Prijavite se v FastComments in pojdite na <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">stran za konfiguracijo LTI 1.3</a>.

Če vaš račun še nima dostopa do LTI, boste videli sporočilo "LTI not enabled for this account" – za omogočitev na vašem paketu se obrnite na podporo.

#### Izberite platformo (neobvezno)

Pod **Generate a Dynamic Registration URL** uporabite spustni meni **Platform**, da FastComments pove, s katerim LMS se povezujete:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Druga platforma LTI 1.3

Lahko pa pustite na **Auto-detect**. Platforma se med registracijo prebere iz openid-configuration vašega LMS; spustni meni le nastavi prikazano oznako za nastalo konfiguracijo.

#### Ustvarite URL

Kliknite **Generate URL**. FastComments ustvari enkratni registracijski žeton in prikaže URL, ki izgleda takole:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopirajte ga. Ta URL:

- Je **za enkratno uporabo** – ko ga vaš LMS uspešno pokliče, je žeton porabljen.
- Poteče po **30 minutah**, če ni uporabljen.
- Naj ostane zaseben – kdorkoli z URL-jem lahko v teh 30 minutah registrira orodje proti vašemu najemniku.

#### Obstoječe konfiguracije

Ko se registracija uspešno zaključi, se nova konfiguracija prikaže v tabeli **Existing Configurations** na isti strani, s podatki Platform, Issuer, Client ID in Status. Konfiguracije lahko iz te tabele izbrišete, če kdaj potrebujejo odjavo.