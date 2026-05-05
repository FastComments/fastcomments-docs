Ponekad FastComments mora poslati e-poruke vašim korisnicima, posebno ako ne koristite Secure SSO.

Primjeri toga uključuju potvrdu njihovog računa ili aktivnosti pri prvom komentiranju. FastComments će im također slati obavijesti o odgovorima na njihove komentare.

Kada FastComments šalje e-poruke vašim korisnicima, upotrijebit ćemo zadano From Name i Email `FastComments Robot` i `noreply@fastcomments.com`.

Također ćemo koristiti naš vlastiti logo u podnožju tih e-poruka.

Ako imate FastComments Flex ili Pro, sve se to može prilagoditi po domeni putem stranice "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Prilikom prilagodbe logotipa koji se prikazuje u e-porukama, pobrinite se da je veličina koju prilažete ista veličina koju želite prikazati u podnožju e-poruke.

### When Customizing The `From Domain`

Ako prilagodite `From Domain`, davatelji e-pošte i klijenti moraju znati da je FastComments ovlašten slati e-poruke u vaše ime. U suprotnom, definiranje `From Domain` bez provođenja koraka u nastavku vjerojatno će rezultirati slanjem e-poruka u neželjenu poštu.

#### 1. Setup SPF

Da biste omogućili FastCommentsu sigurno slanje e-pošte kao vaša domena, pobrinite se da dodate SPF zapis koji nam to dopušta.

Osigurajte da postoje SPF zapisi koji dopuštaju `mail.fastcomments.com` i `sib.fastcomments.com` slanje pošte kao vašu domenu.

Više informacija o tome kako to učiniti nalazi se ovdje: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

Osim SPF-a, trebali biste postaviti DKIM. Nakon što je vaša DNS konfiguracija spremna, možete kliknuti "Show Advanced" na stranici konfiguracija domena kako biste prikazali DKIM postavke po domeni.

Također možete [pozvati API](/guide-api.html#domain-config-structure) kako biste postavili DKIM konfiguraciju.

### Unsubscribe Links

Kada koristite SSO, značajke odjave koje se koriste u e-porukama i obavijestima mogu se prilagoditi [putem DomainConfigs API-ja](/guide-api.html#domain-config-structure).

### Email Link Obfuscation

Ako reputacija domene vaše stranice uzrokuje da obavijesti putem e-pošte završe u neželjenoj pošti, možete usmjeriti gumbe "view comment" kroz `fastcomments.com` umjesto da poveznice vode izravno na vašu stranicu. Pružatelji pošte ocjenjuju svaki link u tijelu e-pošte prema reputaciji odredišta, pa kada je vaša domena označena, same poveznice doprinose rezultatu za neželjenu poštu bez obzira na to koliko je čista vaša konfiguracija slanja.

Omogućite ovo pod "Show Advanced" na stranici My Domains, u odjeljku "Email Link Obfuscation". Postavka je po domeni.

Kada je omogućeno, poveznice u mention, reply, new-comment, subscribed-page, profile-comment i digest e-porukama prepisuju se u kratke tokene koji preusmjeravaju na izvornu stranicu pri kliku. Odredište je vezano za vaš tenant: preusmjeravanje prosljeđuje samo na URL-ove čiji host odgovara jednoj od vaših konfiguriranih domena, a tokeni automatski istječu nakon 30 dana.

Iskustvo nakon klika ostaje nepromijenjeno. Čitatelji i dalje dolaze na vašu stranicu s komentarom pomaknutim u vidljivo stanje.