FastComments provjerava zahtjeve prema vašem računu kako bi vidio dolaze li s vaše stranice. Ovo je razlog zašto
moramo znati na koju stranicu, ili stranice, želite instalirati FastComments.

FastComments podržava autentifikaciju putem domene, kao i poddomena.

Uzmimo stranicu `https://example.com`. U ovom slučaju, "`example.com`" je domena. `example.com` podržava i `example.com`, i `www.example.com`. Nazivat ćemo "www" "poddomenom".

Na primjer:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Ako koristite platformu za bloganje, i dobili ste poddomenu, željeli biste
dodati **punu domenu uključujući poddomenu** na svoj račun, na primjer: `cats.blogger.com`.

Domene možemo dodati na naš račun posjetom stranice `My Domains` i klikom na `Add a Domain` pri dnu:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Tijekom probnog razdoblja, **domene se automatski dodaju na vaš račun** kada zahtjevi dolaze s tih domena. Međutim,
nakon tog razdoblja moraju se dodati izričito radi sigurnosti. Trebali biste primiti e-poštu kada se ovo automatsko ponašanje dogodi.

Ne morate dodavati `localhost` za lokalni razvoj - on je dopušten prema zadanim postavkama.

#### Putem API-ja

Domene se također mogu dodati i konfigurirati [putem DomainConfigs API-ja](/guide-api.html#domain-config-structure).