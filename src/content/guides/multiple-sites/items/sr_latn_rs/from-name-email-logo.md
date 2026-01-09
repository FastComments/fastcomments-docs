Ponekad FastComments mora da šalje e-poruke vašim korisnicima, posebno ako ne koristite Secure SSO.

Primeri uključuju verifikaciju njihovog naloga ili aktivnosti kada komentarišu prvi put. FastComments će im takođe slati obaveštenja o odgovorima na njihove komentare.

Kada FastComments šalje e-poruke vašim korisnicima, koristićemo podrazumevano From ime i adresu e-pošte `FastComments Robot` i `noreply@fastcomments.com`.

Takođe ćemo koristiti naš logo u podnožju tih e-poruka.

Ako imate FastComments Flex ili Pro, sve ovo se može prilagoditi na nivou domena preko stranice "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Prilagođavanje From imena, e-pošte i logotipa' app-screenshot-end]

Prilikom prilagođavanja logotipa koji se prikazuje u e-porukama, proverite da je veličina koju otpremate ista kao veličina koju želite da se prikaže u podnožju e-poruke.

### Prilikom prilagođavanja `From Domain`

Ako prilagodite `From Domain`, provajderi e-pošte i klijenti moraju znati da je FastComments ovlašćen da šalje poruke u vaše ime. U suprotnom, definisanje `From Domain` bez praćenja sledećih koraka verovatno će dovesti do toga da poruke završe u neželjenoj pošti (spam).

#### 1. Podešavanje SPF

Da biste omogućili FastComments‑u da bezbedno šalje e-poruke u ime vašeg domena, dodajte SPF zapis koji nam to dozvoljava.

Proverite da postoje SPF zapisi koji dozvoljavaju da `mail.fastcomments.com` i `sib.fastcomments.com` šalju poštu u ime vašeg domena.

Više informacija o tome kako to uraditi potražite ovde: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Podešavanje DKIM

Pored SPF-a, trebalo bi da podesite DKIM. Kada je vaša DNS konfiguracija spremna, možete kliknuti "Pokaži napredne opcije" na stranici konfiguracije domena da prikažete DKIM podešavanja po domenu.

Takođe možete [pozvati API](/guide-api.html#domain-config-structure) da podesite DKIM konfiguraciju.

### Linkovi za odjavu

Kada koristite SSO, funkcije za odjavu koje se koriste u e-porukama i obaveštenjima mogu se prilagoditi [putem DomainConfigs API](/guide-api.html#domain-config-structure).