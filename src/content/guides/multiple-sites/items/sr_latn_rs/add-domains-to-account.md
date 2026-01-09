FastComments potvrđuje zahteve prema vašem nalogu kako bi se utvrdilo da dolaze sa vašeg sajta. Zbog toga
moramo znati na koji sajt, ili sajtove, želite da instalirate FastComments.

FastComments podržava autentifikaciju pomoću domena, kao i poddomena.

Uzmimo za primer sajt `https://example.com`. U tom slučaju, "`example.com`" je domen. `example.com` podržava i `example.com`, i `www.example.com`. Nazvaćemo "www" poddomenom.

Za primer:

- Da biste dozvolili samo `blog.example.com`:
  - Dodajte `blog.example.com` u vaše domene.
- Da biste dozvolili `www.example.com`, `somesite.example.com` i `example.com`:
  - Dodajte `example.com` u vaše domene.
  - Ovo se naplaćuje kao posedovanje **jednog domena** povezanog sa vašim nalogom.
- Sada možete dodati wildcard poddomene, na primer *myname.vercel.app. 
  - Ovo se naplaćuje kao posedovanje **jednog domena** povezanog sa vašim nalogom.

Ako koristite platformu za blogovanje, i dobijete poddomen, trebalo bi
da dodate **puni domen uključujući poddomen** na vaš nalog, na primer: `cats.blogger.com`.

Možemo dodati domene na naš nalog posetom stranice `My Domains` i klikom na `Add a Domain` na dnu:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Tokom probnog perioda, **domene se automatski dodaju na vaš nalog** kada zahtevi dolaze sa navedenih domena. Međutim,
posle tog vremena moraju biti eksplicitno dodati zbog bezbednosti. Trebalo bi da dobijete email kada se ovo automatsko ponašanje dogodi.

Ne morate dodavati `localhost` za lokalni razvoj - on je dozvoljen po defaultu.

#### Putem API-ja

Domeni se takođe mogu dodati i konfigurisati [putem DomainConfigs API-ja](/guide-api.html#domain-config-structure).