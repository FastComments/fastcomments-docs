### FastComments API

FastComments pruža API za interakciju s mnogim resursima. Izradite integracije s našom platformom ili čak izradite vlastite klijente!

U ovoj dokumentaciji pronaći ćete sve podržane resurse API-ja dokumentirane s njihovim vrstama zahtjeva i odgovora.

Za Enterprise korisnike, sav pristup API-ju bilježi se u revizijskom zapisu.

### Generirani SDK-ovi

FastComments sada generira [API Spec](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije potpuno, ali uključuje mnoge API-je).

Također sada imamo SDK-ove za popularne jezike:

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

API se autentificira prosljeđivanjem vašeg [api key](https://fastcomments.com/auth/my-account/api-secret) kao `X-API-KEY` zaglavlja ili `API_KEY` parametra upita. Također ćete trebati vaš `tenantId` za izvođenje API poziva. To se može preuzeti s iste stranice kao i vaš API ključ.

### Napomena o sigurnosti

Ove rute su namijenjene pozivanju s **poslužitelja**. __NE POZIVAJTE__ ih iz preglednika. To će otkriti vaš API ključ – što će omogućiti potpun pristup vašem računu svima koji mogu vidjeti izvorni kod stranice!

#### Opcija autentifikacije 1 - Zaglavlja

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opcija autentifikacije 2 - Parametri upita

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Opcija autentifikacije 3 - OAuth Bearer token

- Header: `Authorization: Bearer fcat_...`

Aplikacije koje se povezuju putem [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) dobivaju token putem OAuth-a umjesto API ključa. Taj token funkcionira na svakom krajnjem točku ovdje. Najamnik je impliciran tokenom, pa je `tenantId` opcionalan, ali mora odgovarati tokenu ako je naveden. `GET` zahtjevi trebaju `read` opseg, a svi ostali metodi trebaju `write` opseg. Otkrivanje započinje na `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Čitanje vlastitih zapisa

FastComments pruža Active-Active dostupnost. Zahtjevi iz vašeg podatkovnog centra usmjeravaju se na najbližu točku prisutnosti. Ovo je automatsko i obično možete primijetiti semantiku čitanja vlastitog zapisa. Ako želite biti sigurni da čitate vlastite zapise, možete fiksirati svoje zahtjeve na određenu regiju koristeći tu regiju kao API host (međutim, to obično nije potrebno za većinu integracija):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Napomena: ako to učinite, možda ćete željeti definirati rezervni sustav, jer smo u prošlosti ukinuli ulazne čvorove i koristimo nove nazive za prebacivanje.