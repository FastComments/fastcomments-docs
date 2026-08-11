Ponekad FastComments mora slati e‑mailove vašim korisnicima, posebno ako ne koristite Secure SSO.

Primjeri uključuju provjeru njihovog računa ili aktivnosti prilikom komentiranja po prvi put. FastComments će im također slati obavijesti o odgovorima na njihove komentare.

Kada FastComments šalje e‑mailove vašim korisnicima, koristit ćemo zadano ime pošiljatelja i e‑mail `FastComments Robot` i `noreply@fastcomments.com`.

Također ćemo koristiti naš vlastiti logo u podnožju tih e‑mailova.

Ako imate FastComments Flex ili Pro, sve se to može prilagoditi po domeni putem stranice "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Obrazac za postavke e‑mailova po domeni s poljima za ime pošiljatelja, e‑mail pošiljatelja i učitavanje loga'; title='Prilagodba imena pošiljatelja, e‑mail adrese i loga' app-screenshot-end]

Pri prilagodbi loga koji se prikazuje u e‑mailovima, provjerite da je veličina koju učitavate ista veličina koju želite prikazati u podnožju e‑maila.

### Prilikom prilagodbe `From Domain`

Ako prilagodite `From Domain`, pružatelji e‑mail usluga i klijenti moraju znati da je FastComments ovlašten slati e‑mailove u vaše ime. Inače, definiranje `From Domain` bez praćenja dolje navedenih koraka vjerojatno će rezultirati slanjem e‑mailova u spam.

#### 1. Postavljanje SPF-a

Kako bi FastComments mogao sigurno slati e‑mailove kao vaša domena, osigurajte da dodate SPF zapis koji nam to omogućuje.

Provjerite da postoje SPF zapisi koji omogućuju `mail.fastcomments.com` i `sib.fastcomments.com` slati poštu kao vaša domena.

Više informacija o tome kako to učiniti možete pronaći ovdje: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Postavljanje DKIM-a

Uz SPF, trebali biste postaviti DKIM. Kada je vaša DNS konfiguracija spremna, možete kliknuti "Show Advanced" na stranici postavki domena kako biste prikazali DKIM postavke po domeni.

Također možete [pozvati API](/guide-api.html#domain-config-structure) za postavljanje DKIM konfiguracije.

### Poveznice za odjavu

Kada koristite SSO, značajke odjave koje se koriste u e‑mailovima i obavijestima mogu se prilagoditi [putem DomainConfigs API-ja](/guide-api.html#domain-config-structure).

### Obfuskacija poveznica u e‑mailovima

Ako reputacija domene vaše web‑stranice uzrokuje da obavijesni e‑mailovi završe u spamu, možete usmjeriti tipke "view comment" preko `fastcomments.com` umjesto da izravno povezuju na vašu stranicu. Pružatelji poštanskih sandučića ocjenjuju svaku poveznicu u tijelu e‑maila prema reputaciji odredišta, pa kada je vaša domena označena, same poveznice doprinose spam ocjeni, neovisno o tome koliko je vaša konfiguracija slanja čista.

Omogućite ovo pod "Show Advanced" na stranici My Domains, u odjeljku "Email Link Obfuscation". Postavka je po domeni.

Kada je omogućeno, poveznice u e‑mailovima za spominjanje, odgovor, novi komentar, pretplaćenu stranicu, komentar profila i sažetak prepisuju se u kratke tokenove koji preusmjeravaju na izvornu stranicu pri kliku. Odredište je vezano uz vaš najam: preusmjeravanje vodi samo na URL‑ove čiji host odgovara jednoj od vaših konfiguriranih domena, a tokeni automatski istječu nakon 30 dana.

Iskustvo klika ostaje nepromijenjeno. Čitatelji i dalje dolaze na vašu stranicu s komentarom pomaknutim u pogled.