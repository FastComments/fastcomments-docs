### FastComments API

FastComments pruža API za interakciju sa mnogim resursima. Izgradite integracije sa našom platformom, ili čak napravite svoje klijente!

U ovoj dokumentaciji naći ćete sve podržane resurse API-ja dokumentovane sa njihovim tipovima zahteva i odgovora.

Za Enterprise korisnike, sav pristup API-ju se beleži u Dnevniku audita.

### Generisani SDK-ovi

FastComments sada generiše [API specifikaciju](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije kompletno, ali uključuje mnoge API-je).

We also now have SDKs for popular languages:

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

API se autentifikuje prosleđivanjem vašeg [API ključa](https://fastcomments.com/auth/my-account/api-secret) kao ili `X-API-KEY` zaglavlje ili `API_KEY` parametar upita. Takođe će vam trebati vaš `tenantId` za pozive API-ju. Možete ga pronaći na istoj stranici kao i vaš API ključ.

### Sigurnosna napomena

Ove rute su namenjene da se pozivaju sa **servera**. __NE POZIVAJTE__ ih iz pregledača. To će otkriti vaš API ključ — to će omogućiti potpun pristup vašem nalogu svima koji mogu videti izvorni kod stranice!

#### Opcija autentifikacije 1 - Zaglavlja

- Zaglavlje: `X-API-KEY`
- Zaglavlje: `X-TENANT-ID`

#### Opcija autentifikacije 2 - Parametri upita

- Parametar upita: `API_KEY`
- Parametar upita: `tenantId`