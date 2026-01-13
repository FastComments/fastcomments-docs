FastComments preveri overjenost zahtev do vašega računa, da ugotovi, ali prihajajo z vašega spletnega mesta. Zato moramo vedeti, na katero spletno mesto ali spletna mesta želite namestiti FastComments.

FastComments podpira overjanje na podlagi domene, pa tudi poddomen.

Vzemimo spletno mesto `https://example.com`. V tem primeru je "`example.com`" domena. `example.com` podpira tako `example.com` kot `www.example.com`. "www" bomo imenovali "poddomena".

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Če uporabljate blogersko platformo in ste dobili poddomeno, boste želeli dodati **celotno domeno vključno s poddomeno** v vaš račun, na primer: `cats.blogger.com`.

Domene lahko dodamo v naš račun tako, da obiščemo stran `My Domains` in na dnu kliknemo `Add a Domain`:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Med preizkusnim obdobjem se **domene samodejno dodajo v vaš račun**, ko zahtevki prihajajo s teh domen. Vendar pa jih je po tem obdobju potrebno zaradi varnosti dodati izrecno. Ob tem samodejnem dejanju bi morali prejeti e-pošto.

Za lokalni razvoj vam ni treba dodajati `localhost` - to je privzeto dovoljeno.

#### Prek API-ja

Domene je mogoče tudi dodati in konfigurirati [prek DomainConfigs API](/guide-api.html#domain-config-structure).