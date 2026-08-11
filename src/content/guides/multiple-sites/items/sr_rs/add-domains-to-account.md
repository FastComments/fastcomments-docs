FastComments autentifikuje zahteve ka vašem nalogu da vidi da dolaze sa vašeg sajta. Zato  
moramo da znamo koji sajt ili sajtovi želite da instalirate FastComments na.

FastComments podržava autentifikaciju putem domena, kao i poddomena.

Uzmimo sajt `https://example.com`. U ovom slučaju, "`example.com`" je domen. `example.com` podržava i `example.com`, i `www.example.com`. "www" ćemo nazvati "poddomen".

Na primer:

- Da biste dozvolili samo `blog.example.com`:
  - Dodajte `blog.example.com` u vaše domene.
- Da biste dozvolili `www.example.com`, `somesite.example.com` i `example.com`:
  - Dodajte `example.com` u vaše domene.
  - Ovo se naplaćuje kao **jedan domen** povezan sa vašim nalogom.
- Sada možete dodati wildcard poddomene, na primer *myname.vercel.app.  
  - Ovo se naplaćuje kao **jedan domen** povezan sa vašim nalogom.

Ako koristite platformu za blogovanje i dobijete poddomen, treba da  
dodate **puni domen uključujući poddomen** na vaš nalog, na primer: `cats.blogger.com`.

Možemo dodati domene na naš nalog tako što ćemo posetiti stranicu `My Domains` i kliknuti na `Add a Domain` na dnu:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Stranica My Domains prikazuje domene na nalogu, sa dugmetom Dodaj domen na dnu'; title='Stranica My Domains' app-screenshot-end]

Tokom probnog perioda, **domene se automatski dodaju na vaš nalog** kada zahtevi dolaze sa tih domena. Međutim,  
nakon tog vremena moraju se dodati eksplicitno iz sigurnosnih razloga. Trebalo bi da dobijete email kada se ovo automatsko ponašanje desi.

Ne morate da dodajete `localhost` za lokalni razvoj – on je podrazumevano dozvoljen.

#### Preko API-ja

Domene se takođe mogu dodati i konfigurisati [preko DomainConfigs API](/guide-api.html#domain-config-structure).