FastComments authenticert verzoeken naar uw account om te zien dat ze van uw site komen. Daarom moeten we weten welke site(s) u FastComments op wilt installeren.

FastComments ondersteunt authenticatie via domein, evenals subdomeinen.

Laten we de site `https://example.com` nemen. In dit geval is "`example.com`" het domein. `example.com` ondersteunt zowel `example.com` als `www.example.com`. We zullen de "www" het "subdomein" noemen.

Bijvoorbeeld:

- Om alleen `blog.example.com` toe te staan:
  - Voeg `blog.example.com` toe aan uw domeinen.
- Om `www.example.com`, `somesite.example.com` en `example.com` toe te staan:
  - Voeg `example.com` toe aan uw domeinen.
  - Dit wordt gefactureerd als **één domein** dat aan uw account is gekoppeld.
- U kunt nu wildcard-subdomeinen toevoegen, bijvoorbeeld *myname.vercel.app.
  - Dit wordt gefactureerd als **één domein** dat aan uw account is gekoppeld.

Als u een blogplatform gebruikte en u een subdomein kreeg, wilt u het **volledige domein inclusief het subdomein** aan uw account toevoegen, bijvoorbeeld: `cats.blogger.com`.

We kunnen domeinen aan ons account toevoegen door de `My Domains`-pagina te bezoeken en onderaan op `Add a Domain` te klikken:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='My Domains-pagina die de domeinen op het account weergeeft, met de knop Add a Domain onderaan'; title='De My Domains-pagina' app-screenshot-end]

Tijdens de proefperiode worden **domeinen automatisch aan uw account toegevoegd** wanneer verzoeken van die domeinen komen. Na deze periode moeten ze echter expliciet worden toegevoegd voor de beveiliging. U zou een e‑mail moeten ontvangen wanneer dit geautomatiseerde gedrag optreedt.

U hoeft **niet** `localhost` toe te voegen voor lokale ontwikkeling – dit is standaard toegestaan.

#### Via de API

Domeinen kunnen ook worden toegevoegd en geconfigureerd [via de DomainConfigs API](/guide-api.html#domain-config-structure).