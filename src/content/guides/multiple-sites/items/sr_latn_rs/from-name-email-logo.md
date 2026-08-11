Ponekad FastComments mora da pošalje e‑mail vašim korisnicima, posebno ako ne koristite Secure SSO.

Primeri uključuju verifikaciju njihovog naloga ili aktivnosti kada prvi put komentarišu. FastComments
će im takođe slati obaveštenja o odgovorima na njihove komentare.

Kada FastComments šalje e‑mailove vašim korisnicima, koristićemo podrazumevano ime pošiljaoca i e‑mail `FastComments Robot` i `noreply@fastcomments.com`.

Takođe ćemo koristiti naš logo u podnožju ovih e‑mailova.

Ako imate FastComments Flex ili Pro, sve ovo se može prilagoditi po domenu putem stranice „My Domains“:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Obrazac za podešavanje e‑mailova po domenu sa poljima Za ime, Za e‑mail i otpremanje loga'; title='Prilagođavanje imena pošiljaoca, e‑mail adrese i loga' app-screenshot-end]

Kada prilagođavate logo koji se prikazuje u e‑mailovima, uverite se da je veličina koju otpremate ista kao veličina koju želite da se prikaže u podnožju e‑maila.

### Kada prilagođavate `From Domain`

Ako prilagodite `From Domain`, provajderi e‑maila i klijenti moraju da znaju da je FastComments ovlašćen da šalje e‑mailove u vaše ime. U suprotnom, definisanje `From Domain` bez praćenja koraka ispod verovatno će rezultirati slanjem e‑mailova u spam.

#### 1. Postavljanje SPF

Da biste omogućili FastComments-u da sigurno šalje e‑mailove kao vaš domen, uverite se da ste dodali SPF zapis koji nam to dozvoljava.

Uverite se da postoje SPF zapisi koji dozvoljavaju `mail.fastcomments.com` i `sib.fastcomments.com` da šalju poštu kao vaš domen.

Više informacija o tome možete pronaći ovde: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Postavljanje DKIM

Pored SPF-a, treba da postavite DKIM. Kada vaša DNS konfiguracija bude spremna, možete kliknuti „Prikaži napredno“ na stranici za konfiguraciju domena
da biste prikazali DKIM podešavanja po domenu.

Takođe možete [pozvati API](/guide-api.html#domain-config-structure) da postavite DKIM konfiguraciju.

### Linkovi za odjavu

Kada koristite SSO, funkcije odjave koje se koriste u e‑mailovima i obaveštenjima mogu se prilagoditi [preko DomainConfigs API-ja](/guide-api.html#domain-config-structure).

### Obfuskacija linkova u e‑mailu

Ako reputacija domena vašeg sajta uzrokuje da obaveštajni e‑mailovi završe u spamu, možete usmeriti dugmad „view comment“ preko `fastcomments.com` umesto da direktno povežete na vašu stranicu. Provajderi poštanskih sandučića ocenjuju svaki link u telu e‑maila prema reputaciji odredišta, pa kada je vaš domen označen, same veze doprinose spam skor-u bez obzira koliko je vaša poštanska konfiguracija čista.

Omogućite ovo pod „Prikaži napredno“ na stranici My Domains, u odeljku „Obfuskacija linkova u e‑mailu“. Podešavanje je po domenu.

Kada je omogućeno, linkovi u e‑mailovima za pominjanje, odgovor, novi komentar, pretplaćenu stranicu, komentar profila i sažetke se prepisuju u kratke tokene koji preusmeravaju na originalnu stranicu po kliku. Odredište je vezano za vaš tenant: preusmeravanje vodi samo na URL‑ove čiji host odgovara jednom od vaših konfigurisanih domena, a tokeni automatski ističu posle 30 dana.

Iskustvo klika ostaje nepromenjeno. Čitaoci i dalje dolaze na vašu stranicu sa komentarom pomerenim u vidno polje.