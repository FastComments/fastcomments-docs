Ponekad FastComments mora slati e-poštu vašim korisnicima, posebno ako ne koristite Secure SSO.

Primjeri uključuju provjeru njihovog računa ili aktivnosti pri prvom komentiranju. FastComments
will also send them notifications for replies to their comments.

Kad FastComments šalje e-poštu vašim korisnicima, koristit ćemo zadano ime pošiljatelja i adresu e-pošte `FastComments Robot` i `noreply@fastcomments.com`.

Također ćemo koristiti vlastiti logotip u podnožju tih e-pošta.

Ako imate FastComments Flex ili Pro, sve se to može prilagoditi po domeni putem stranice "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Prilikom prilagođivanja logotipa koji se prikazuje u e-poštama, pazite da veličina koju učitavate bude ista veličina koju želite prikazati u podnožju e-pošte.

### Prilikom prilagodbe `From Domain`

Ako prilagodite `From Domain`, davatelji i klijenti e-pošte moraju znati da je FastComments ovlašten slati e-poštu u vaše ime. U suprotnom,
definiranje `From Domain` bez praćenja donjih koraka vjerojatno će rezultirati time da će e-pošta završiti u neželjenoj pošti.

#### 1. Setup SPF

Da biste omogućili FastCommentsu da sigurno šalje e-poštu u ime vaše domene, osigurajte da dodate SPF zapis koji nam to dopušta.

Osigurajte da postoje SPF zapisi koji dopuštaju `mail.fastcomments.com` i `sib.fastcomments.com` slanje e-pošte u ime vaše domene.

Više informacija o tome kako to učiniti nalazi se ovdje: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

Osim SPF-a, trebali biste postaviti i DKIM. Nakon što je vaša DNS konfiguracija spremna, možete kliknuti "Prikaži napredno" na stranici s konfiguracijama domene
kako biste prikazali DKIM postavke po domeni.

Također možete [invoke the API](/guide-api.html#domain-config-structure) to set DKIM configuration.

### Unsubscribe Links

Kada koristite SSO, značajke odjave koje se koriste u e-poštama i obavijestima mogu se prilagoditi [via the DomainConfigs API](/guide-api.html#domain-config-structure).