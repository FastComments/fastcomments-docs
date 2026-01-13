### FastComments API

FastComments pruža API za interakciju sa mnogim resursima. Izgradite integracije sa našom platformom, ili čak napravite sopstvene klijente!

U ovoj dokumentaciji pronaći ćete sve resurse koje podržava API, dokumentovane sa njihovim tipovima zahteva i odgovora.

Za Enterprise korisnike, sav pristup API-ju se beleži u Audit logu.

### Generisani SDK-i

FastComments sada generiše [API specifikacija](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije kompletno, ali uključuje mnoge API-je).

Takođe sada imamo SDK-ove za popularne jezike:

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

API se autentifikuje prosleđivanjem vašeg [API ključa](https://fastcomments.com/auth/my-account/api-secret) kao ili `X-API-KEY` headera ili `API_KEY` query parametra. Takođe će vam biti potreban vaš `tenantId` za pozivanje API-ja. To možete preuzeti sa iste stranice kao i vaš API ključ.

### Napomena o bezbednosti

Ove rute su namenjene da budu pozivane sa **servera**. __NE POZIVAJTE__ ih iz pregledača. Ako to uradite, izložićete vaš API ključ — to će omogućiti potpuni pristup vašem nalogu svima koji mogu da vide izvorni kod stranice!

#### Opcija autentifikacije 1 - Headeri

- Zaglavlje: `X-API-KEY`
- Zaglavlje: `X-TENANT-ID`

#### Opcija autentifikacije 2 - Parametri upita

- Parametar upita: `API_KEY`
- Parametar upita: `tenantId`