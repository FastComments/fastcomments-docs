### API FastComments

FastComments nudi API za interakcijo z mnogimi viri. Zgradite integracije z našo platformo ali celo ustvarite svoje lastne odjemalce!

V tej dokumentaciji najdete vse vire, ki jih API podpira, dokumentirane z njihovimi tipi zahtevkov in odgovorov.

Za Enterprise naročnike je ves dostop do API zabeležen v revizijskem dnevniku.

### Generirani SDK-ji

FastComments sedaj generira [API specifikacijo](https://fastcomments.com/js/swagger.json) iz naše kode (to še ni popolno, vendar vključuje mnoge API-je).

Imamo tudi SDK-je za priljubljene jezike:

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

### Avtentikacija

API se avtenticira tako, da posredujete svoj [API ključ](https://fastcomments.com/auth/my-account/api-secret) bodisi kot `X-API-KEY` header ali `API_KEY` query parameter. Za klice API boste potrebovali tudi svoj `tenantId`
za izvajanje klicev API. To lahko pridobite na isti strani kot svoj API ključ.

### Varnostno opozorilo

Te poti so namenjene klicanju z **strežnika**. __NE__ jih kličite iz brskalnika. S tem boste razkrili svoj API ključ - to bo vsakomur, ki si lahko ogleda izvorno kodo strani, omogočilo popoln dostop do vašega računa!

#### Authentication Option One - Headers

- Glava: `X-API-KEY`
- Glava: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Parameter poizvedbe: `API_KEY`
- Parameter poizvedbe: `tenantId`

---