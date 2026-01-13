### FastComments API

FastComments pruža API za interakciju sa mnogim resursima. Napravite integracije sa našom platformom, ili čak izradite vlastite klijente!

U ovoj dokumentaciji naći ćete sve podržane resurse koje API dokumentuje zajedno sa tipovima zahtjeva i odgovora.

Za Enterprise korisnike, sav pristup API-ju se bilježi u Dnevniku revizije.

### Generisani SDK-ovi

FastComments sada generiše [API Spec](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije kompletno, ali uključuje mnoge API-je).

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

API se autentifikuje tako što proslijedite svoj [API ključ](https://fastcomments.com/auth/my-account/api-secret) kao ili `X-API-KEY` zaglavlje ili `API_KEY` parametar upita. Također će vam trebati vaš `tenantId` za pozive API-ja. To se može dobiti na istoj stranici kao i vaš API ključ.

### Napomena o sigurnosti

Ove rute su namijenjene da se pozivaju sa **servera**. __NEMOJTE__ ih pozivati iz preglednika. Time ćete otkriti vaš API ključ - to će omogućiti potpuni pristup vašem nalogu svima koji mogu vidjeti izvorni kod stranice!

#### Authentication Option One - Headers

- Zaglavlje: `X-API-KEY`
- Zaglavlje: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Parametar upita: `API_KEY`
- Parametar upita: `tenantId`