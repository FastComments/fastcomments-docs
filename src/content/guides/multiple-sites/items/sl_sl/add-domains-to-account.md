FastComments overja zahteve do vašega računa, da vidi, da prihajajo z vaše strani. Zato moramo vedeti, na kateri strani ali straneh želite namestiti FastComments.

FastComments podpira overjanje prek domene, kot tudi poddomen.

Vzemimo stran `https://example.com`. V tem primeru je "`example.com`" domena. `example.com` podpira tako `example.com` kot `www.example.com`. "www" bomo poimenovali "poddomena".

Na primer:

- Za dovoljenje samo `blog.example.com`:
  - Dodajte `blog.example.com` v svoje domene.
- Za dovoljenje `www.example.com`, `somesite.example.com` in `example.com`:
  - Dodajte `example.com` v svoje domene.
  - To se zaračuna kot **ena domena**, povezana z vašim računom.
- Zdaj lahko dodate nadomestne poddomene, na primer *myname.vercel.app.
  - To se zaračuna kot **ena domena**, povezana z vašim računom.

Če uporabljate platformo za bloganje in ste dobili poddomeno, boste želeli dodati **celotno domeno, vključno s poddomeno**, v svoj račun, na primer: `cats.blogger.com`.

Domene lahko dodamo v svoj račun tako, da obiščemo stran `My Domains` in na dnu kliknemo `Add a Domain`:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Stran Moji domene, ki prikazuje domene na računu, z gumbom Dodaj domeno na dnu'; title='Stran Moji domene' app-screenshot-end]

Med preizkusnim obdobjem se **domene samodejno dodajo vašemu računu**, ko zahteve prihajajo s teh domen. Vendar pa jih je po tem času treba iz varnostnih razlogov dodati ročno. Ko se to samodejno vedenje zgodi, bi morali prejeti e‑pošto.

Za lokalni razvoj **ni** potrebno dodajati `localhost` – je privzeto dovoljen.

#### Prek API-ja

Domene je mogoče tudi dodati in konfigurirati [prek DomainConfigs API](/guide-api.html#domain-config-structure).