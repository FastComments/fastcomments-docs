### FastComments API

FastComments pruža API za interakciju s mnogim resursima. Izgradite integracije s našom platformom ili čak izradite vlastite klijente!

U ovoj dokumentaciji naći ćete sve podržane resurse API‑ja dokumentirane s njihovim tipovima zahtjeva i odgovora.

Za Enterprise korisnike, sav pristup API‑ju bilježi se u Dnevniku revizija.

### Generirani SDK‑ovi

FastComments sada generira [API Spec](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije potpuno, ali uključuje mnoge API‑e).

Također sada imamo SDK‑ove za popularne jezike:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Autentifikacija

API se autentificira prosljeđivanjem vašeg [api ključa](https://fastcomments.com/auth/my-account/api-secret) kao `X-API-KEY` zaglavlja ili `API_KEY` parametra upita. Također će vam trebati vaš `tenantId` za izvođenje API poziva. On se može preuzeti s iste stranice kao i vaš api ključ.

### Napomena o sigurnosti

Ove rute su namijenjene pozivanju s **poslužitelja**. __NE__ pozivajte ih iz preglednika. To će otkriti vaš API ključ – što će omogućiti potpun pristup vašem računu svima koji mogu vidjeti izvorni kod stranice!

#### Opcija autentifikacije jedan – Zaglavlja

- Zaglavlje: `X-API-KEY`
- Zaglavlje: `X-TENANT-ID`

#### Opcija autentifikacije dva – Parametri upita

- Parametar upita: `API_KEY`
- Parametar upita: `tenantId`

#### Opcija autentifikacije tri – OAuth Bearer token

- Zaglavlje: `Authorization: Bearer fcat_...`

Aplikacije trećih strana poput Zapiera i klijenti [MCP poslužitelja](https://docs.fastcomments.com/guide-llm-kit.html) dobivaju token putem OAuth‑a umjesto API ključa. Taj token funkcionira na svakom krajnjem točki ovdje. Najmodavac je impliciran tokenom, pa je `tenantId` opcionalan, ali mora odgovarati tokenu ako je naveden. `GET` zahtjevi trebaju `read` opseg, a sve ostale metode `write` opseg. Cijeli tijek, uključujući registraciju klijenta, PKCE, osvježavanje i opoziv, dokumentiran je pod [OAuth Authorization](#oauth). Otkrivanje započinje na `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Čitanje vlastitih zapisa

FastComments pruža Active‑Active dostupnost. Zahtjevi iz vašeg podatkovnog centra usmjeravaju se na [najbližu točku prisutnosti](https://sophon.fastcomments.com/) u odnosu na vas. Ovo je automatsko i obično možete primijetiti semantiku čitanja‑vašeg‑zapisa. Ako želite biti sigurni da čitate vlastite zapise, možete učvrstiti svoje zahtjeve na određenu regiju koristeći tu regiju kao API host (iako to obično nije potrebno za većinu integracija):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Napomena: ako to učinite, možda ćete željeti definirati rezervni plan, jer smo u prošlosti ukinuli ulazne čvorove i koristimo nove nazive za prebacivanje.