Ponekad FastComments mora da pošalje e-poštu vašim korisnicima, posebno ako ne koristite Secure SSO.

Primeri za to uključuju verifikaciju naloga ili aktivnosti kada komentarišu prvi put. FastComments će im takođe slati obaveštenja za odgovore na njihove komentare.

Kada FastComments pošalje e-poštu vašim korisnicima, koristićemo podrazumevano From Name i Email `FastComments Robot` i `noreply@fastcomments.com`.

Takođe ćemo koristiti naš logo u podnožju ovih e-poruka.

Ako imate FastComments Flex ili Pro, sve ovo se može prilagoditi po domenima putem "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Kada prilagođavate logo koji se prikazuje u e-porukama, proverite da veličina koju otpremate odgovara veličini koju želite da prikažete u podnožju e-poruke.

### Kada prilagođavate `From Domain`

Ako prilagodite `From Domain`, provajderi i klijenti e-pošte treba da znaju da je FastComments ovlašćen da šalje e-poštu u ime vašeg domena. U suprotnom,
definisanje `From Domain` bez sprovođenja koraka ispod verovatno će rezultovati time da e-poruke završe u neželjenoj pošti.

#### 1. Setup SPF

Da biste omogućili FastComments-u da bezbedno šalje e-poštu kao vaš domen, obavezno dodajte SPF zapis koji nam to dopušta.

Obezbedite da postoje SPF zapisi koji omogućavaju `mail.fastcomments.com` i `sib.fastcomments.com` da šalju poštu u ime vašeg domena.

Više informacija o tome kako to uraditi nalazi se ovde: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

Pored SPF-a, trebalo bi da podesite i DKIM. Kada je vaša DNS konfiguracija spremna, možete kliknuti na "Show Advanced" na stranici za konfiguraciju domena
da biste prikazali DKIM podešavanja po domenu.

Takođe možete [pozvati API](/guide-api.html#domain-config-structure) da podesite DKIM konfiguraciju.

### Linkovi za odjavu

Kada koristite SSO, funkcije za odjavu koje se koriste u e-porukama i obaveštenjima mogu se prilagoditi [putem DomainConfigs API](/guide-api.html#domain-config-structure).

### Obfuskacija linkova u e-porukama

Ako reputacija domena vašeg sajta uzrokuje da obaveštenja stižu u neželjenu poštu, možete usmeriti dugmad "view comment" preko `fastcomments.com` umesto da povezujete direktno na vašu stranicu. Provajderi poštanskih sandučeta ocenjuju svaki link u telu e-poruke prema reputaciji destinacije, pa kada je vaš domen označen, goli linkovi doprinose spam skoru bez obzira koliko je vaše slanje čisto podešeno.

Omogućite ovo pod "Show Advanced" na My Domains page, u sekciji "Email Link Obfuscation". Podešavanje je po domenu.

Kada je omogućeno, linkovi u mention, reply, new-comment, subscribed-page, profile-comment i digest e-porukama se prepisuju u kratke token-e koji preusmeravaju na originalnu stranicu pri kliku. Destinacija je vezana za vaš tenant: preusmeravanje će proslediti samo URL-ove čiji host odgovara jednom od vaših konfigurisанih domena, i tokeni automatski ističu nakon 30 dana.

Iskustvo pri kliku ostaje nepromenjeno. Čitaoci i dalje stižu na vašu stranicu sa komentarom skrolovanim u vidokrugu.