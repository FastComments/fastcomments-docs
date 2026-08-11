FastComments autentificira zahtjeve na vaš račun kako bi provjerio da dolaze s vaše web stranice. Zato trebamo znati koju stranicu ili stranice želite instalirati FastComments.

FastComments podržava autentifikaciju putem domene, kao i poddomena.

Uzmimo stranicu `https://example.com`. U ovom slučaju, "`example.com`" je domena. `example.com` podržava i `example.com` i `www.example.com`. "www" ćemo nazvati "poddomenom".

Na primjer:

- Za dopuštanje samo `blog.example.com`:
  - Dodajte `blog.example.com` u svoje domene.
- Za dopuštanje `www.example.com`, `somesite.example.com` i `example.com`:
  - Dodajte `example.com` u svoje domene.
  - Ovo se naplaćuje kao **jedna domena** povezana s vašim računom.
- Sada možete dodati wildcard poddomene, na primjer *myname.vercel.app.
  - Ovo se naplaćuje kao **jedna domena** povezana s vašim računom.

Ako koristite platformu za bloganje i dobili ste poddomen, trebali biste dodati **cijelu domenu uključujući poddomen** na svoj račun, na primjer: `cats.blogger.com`.

Domene možemo dodati na svoj račun posjetom stranici `My Domains` i klikom na `Add a Domain` na dnu:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Stranica My Domains koja prikazuje domene na računu, s gumbom Add a Domain na dnu'; title='Stranica My Domains' app-screenshot-end]

Tijekom probnog razdoblja, **domene se automatski dodaju na vaš račun** kada zahtjevi dolaze s tih domena. Međutim, nakon tog vremena moraju se dodati izričito radi sigurnosti. Trebali biste primiti e‑mail kada se ovo automatizirano ponašanje dogodi.

Ne morate dodavati `localhost` za lokalni razvoj – on je dopušten po zadanom.

#### Putem API-ja

Domene se također mogu dodati i konfigurirati [putem DomainConfigs API](/guide-api.html#domain-config-structure).