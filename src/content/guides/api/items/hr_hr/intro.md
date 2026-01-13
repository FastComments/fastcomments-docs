### FastComments API

FastComments pruža API za interakciju s mnogim resursima. Izgradite integracije s našom platformom ili čak napravite vlastite klijente!

U ovoj dokumentaciji pronaći ćete sve podržane resurse koje API dokumentira zajedno s njihovim tipovima zahtjeva i odgovora.

Za Enterprise korisnike, sav pristup API-ju bilježi se u dnevniku revizije.

### Generirani SDK-ovi

FastComments sada generira [API Spec](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije potpuno dovršeno, ali uključuje mnoge API-je).

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

API se autentificira prosljeđivanjem vašeg [API ključa](https://fastcomments.com/auth/my-account/api-secret) kao ili `X-API-KEY` zaglavlje ili `API_KEY` query parametar. Također ćete trebati svoj `tenantId` za pozivanje API-ja. To se može dohvatiti na istoj stranici kao i vaš api ključ.

### Sigurnosna napomena

Ove rute namijenjene su pozivanju s **poslužitelja**. __NEMOJTE__ ih pozivati iz preglednika. To će otkriti vaš API ključ - to će pružiti potpuni pristup vašem računu svima koji mogu pregledati izvorni kod stranice!

#### Opcija autentifikacije 1 - Zaglavlja

- Zaglavlje: `X-API-KEY`
- Zaglavlje: `X-TENANT-ID`

#### Opcija autentifikacije 2 - Parametri upita

- Parametar upita: `API_KEY`
- Parametar upita: `tenantId`

---