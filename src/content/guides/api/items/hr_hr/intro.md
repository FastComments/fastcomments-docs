### FastComments API

FastComments pruža API za interakciju s mnogim resursima. Izradite integracije s našom platformom ili čak izradite vlastite klijente!

U ovoj dokumentaciji pronaći ćete sve podržane resurse API-ja, dokumentirane s njihovim tipovima zahtjeva i odgovora.

Za Enterprise korisnike, sav pristup API-ju bilježi se u zapisniku revizije.

### Generirani SDK-ovi

FastComments sada generira [API specifikaciju](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije dovršeno, ali uključuje mnoge API-je).

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

API se autentificira prosljeđivanjem vaše [api key](https://fastcomments.com/auth/my-account/api-secret) kao ili `X-API-KEY` zaglavlje ili `API_KEY` parametar upita. Također će vam trebati vaš `tenantId` za pozive API-ja. To se može dohvatiti na istoj stranici kao i vaš api key.

### Sigurnosna napomena

Ove rute namijenjene su pozivanju s **poslužitelja**. __NE POZIVAJTE__ ih iz preglednika. Ako ih pozovete iz preglednika, izložiti ćete svoj API ključ — to će omogućiti potpun pristup vašem računu svima koji mogu pregledati izvorni kod stranice!

#### Opcija autentikacije 1 - Zaglavlja

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Opcija autentikacije 2 - Parametri upita

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Čitanje vlastitih zapisa

FastComments omogućuje Active-Active dostupnost. Zahtjevi iz vašeg podatkovnog centra usmjeravaju se na [najbližu točku prisutnosti](https://sophon.fastcomments.com/) u odnosu na vašu. To je automatski, i obično možete uočiti semantiku read-your-write. Ako želite biti sigurni da ćete pročitati vlastite zapise, možete pripeti svoje zahtjeve na određenu regiju koristeći tu regiju kao API host (međutim, ovo obično nije potrebno za većinu integracija):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Imajte na umu da ako to napravite, možda ćete htjeti definirati fallback, jer smo u prošlosti ukinuli ulazne čvorove i koristili nova imena za prebacivanje.

---