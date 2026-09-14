### FastComments API

FastComments pruža API za interakciju sa mnogim resursima. Izgradite integracije sa našom platformom, ili čak izradite sopstvene klijente!

U ovoj dokumentaciji ćete pronaći sve podržane resurse API‑ja dokumentovane sa njihovim tipovima zahteva i odgovora.

Za Enterprise korisnike, sav pristup API‑ju se beleži u revizijskom dnevniku.

### Generisani SDK‑ovi

FastComments sada generiše [API specifikaciju](https://fastcomments.com/js/swagger.json) iz našeg koda (još nije kompletna, ali uključuje mnoge API‑e).

Takođe sada imamo SDK‑ove za popularne jezike:

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

API se autentifikuje prosleđivanjem vašeg [api ključa](https://fastcomments.com/auth/my-account/api-secret) kao `X-API-KEY` zaglavlja ili `API_KEY` parametra upita. Takođe će vam biti potreban `tenantId` za pozivanje API‑ja. Može se preuzeti sa iste stranice kao i vaš api ključ.

### Napomena o bezbednosti

Ove rute su namenjene pozivanju sa **servera**. __NE__ pozivajte ih iz pregledača. To će otkriti vaš API ključ – što omogućava potpun pristup vašem nalogu bilo kome ko može da vidi izvorni kod stranice!

#### Opcija autentifikacije 1 – Zaglavlja

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opcija autentifikacije 2 – Parametri upita

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Opcija autentifikacije 3 – OAuth Bearer token

- Header: `Authorization: Bearer fcat_...`

Third‑party aplikacije kao što su Zapier i klijenti [MCP servera](https://docs.fastcomments.com/guide-llm-kit.html) dobijaju token putem OAuth‑a umesto API ključa. Taj token funkcioniše na svakom krajnjem punktu ovde. Tenant je impliciran tokenom, pa je `tenantId` opcionalan, ali mora da se podudara sa tokenom ako je naveden. `GET` zahtevi zahtevaju `read` opseg, a svi ostali metodi zahtevaju `write` opseg. Ceo tok, uključujući registraciju klijenta, PKCE, osvežavanje i opoziv, dokumentovan je pod [OAuth Authorization](#oauth). Otkrivanje počinje na `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Čitanje sopstvenih upisa

FastComments pruža Active‑Active dostupnost. Zahtevi iz vašeg data‑centra se usmeravaju na [najbližu tačku prisustva](https://sophon.fastcomments.com/) u odnosu na vas. Ovo je automatsko i obično možete primetiti semantiku čitanja‑posle‑pisanja. Ako želite da budete sigurni da čitate sopstvene upise, možete fiksirati svoje zahteve na određenu regiju koristeći tu regiju kao API host (iako to obično nije potrebno za većinu integracija):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Napomena: ako to uradite, možda ćete želeti da definišete rezervni plan, pošto smo u prošlosti ukinuli ulazne čvorove i koristimo nove nazive za prebacivanje.