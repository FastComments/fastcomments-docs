### FastComments API

FastComments pruža API za interakciju sa mnogim resursima. Izgradite integracije sa našom platformom, ili čak izradite svoje klijente!

U ovoj dokumentaciji naći ćete sve podržane resurse API‑ja dokumentovane sa njihovim tipovima zahteva i odgovora.

Za Enterprise korisnike, sav pristup API‑ju se beleži u Audit Logu.

### Generisani SDK‑i

FastComments sada generiše [API Spec](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije kompletno, ali uključuje mnoge API‑e).

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

API se autentifikuje prosleđivanjem vašeg [api ključa](https://fastcomments.com/auth/my-account/api-secret) kao `X-API-KEY` zaglavlja ili `API_KEY` parametra upita. Takođe će vam biti potreban `tenantId` za pravljenje API poziva. On se može preuzeti sa iste stranice kao i vaš api ključ.

### Napomena o bezbednosti

Ove rute su namenjene pozivanju sa **servera**. __NE POZIVAJTE__ ih iz pregledača. To će izložiti vaš API ključ – što omogućava potpun pristup vašem nalogu bilo kome ko može da vidi izvorni kod stranice!

#### Opcija autentifikacije 1 – Zaglavlja

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opcija autentifikacije 2 – Parametri upita

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Opcija autentifikacije 3 – OAuth Bearer token

- Header: `Authorization: Bearer fcat_...`

Aplicacije koje se povezuju preko [MCP servera](https://docs.fastcomments.com/guide-llm-kit.html) dobijaju token putem OAuth‑a umesto API ključa. Taj token radi na svakom krajnjem punktu ovde. Tenant je impliciran tokenom, pa je `tenantId` opcionalan, ali mora da se podudara sa tokenom ako je naveden. `GET` zahtevi zahtevaju `read` opseg, a svi ostali zahtevi zahtevaju `write` opseg. Otkrivanje počinje na `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Čitanje sopstvenih upisa

FastComments pruža Active-Active dostupnost. Zahtevi iz vašeg data‑centra se usmeravaju na [najbližu tačku prisustva](https://sophon.fastcomments.com/) u odnosu na vas. Ovo je automatsko i obično možete da primetite semantiku čitanja‑posle‑pisanja. Ako želite da budete sigurni da čitate svoje sopstvene upise, možete da fiksirate svoje zahteve na određenu regiju koristeći tu regiju kao API host (iako ovo obično nije potrebno za većinu integracija):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Napomena: ako to uradite, možda ćete želeti da definišete rezervni put, pošto smo u prošlosti deprecirali ulazne čvorove i koristimo nove nazive za prebacivanje.