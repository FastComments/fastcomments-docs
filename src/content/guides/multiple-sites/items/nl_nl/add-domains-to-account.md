---
FastComments verifieert verzoeken naar uw account om te controleren of ze van uw site komen. Dit is waarom
we moeten weten op welke site, of sites, u FastComments wilt installeren.

FastComments ondersteunt authenticatie op basis van een domein, evenals subdomeinen.

Laten we de site `https://example.com` nemen. In dit geval is "`example.com`" het domein. `example.com` ondersteunt zowel `example.com`, en `www.example.com`. We noemen de "www" het "subdomein".

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - Dit wordt gefactureerd als het hebben van **één domein** dat aan uw account is gekoppeld.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - Dit wordt gefactureerd als het hebben van **één domein** dat aan uw account is gekoppeld.

Als u een bloggingplatform gebruikte, en u een subdomein kreeg toegewezen, zou u
het **volledige domein inclusief het subdomein** aan uw account willen toevoegen, bijvoorbeeld: `cats.blogger.com`.

We kunnen domeinen aan ons account toevoegen door de `My Domains` pagina te bezoeken en onderaan op `Add a Domain` te klikken:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Tijdens de proefperiode worden **domeinen automatisch aan uw account toegevoegd** wanneer verzoeken vanaf die domeinen binnenkomen. Echter,
na deze periode moeten ze expliciet worden toegevoegd uit veiligheidsoverwegingen. U zou een e-mail moeten ontvangen wanneer dit geautomatiseerde gedrag plaatsvindt.

U hoeft `localhost` niet toe te voegen voor lokale ontwikkeling - het is standaard toegestaan.

#### Via de API

Domeinen kunnen ook worden toegevoegd en geconfigureerd [via de DomainConfigs API](/guide-api.html#domain-config-structure).

---