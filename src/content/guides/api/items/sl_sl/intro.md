### FastComments API

FastComments zagotavlja API za interakcijo z mnogimi viri. Ustvarite integracije z našo platformo ali celo napišite svoje lastne odjemalce!

V tej dokumentaciji boste našli vse vire, ki jih API podpira, dokumentirane z njihovimi tipi zahtevkov in odgovorov.

Za Enterprise stranke se celoten dostop do API beleži v revizijski dnevnik.

### Generirani SDK-ji

FastComments zdaj iz naše kode generira [Specifikacija API](https://fastcomments.com/js/swagger.json) (to še ni popolno, vendar vključuje številne API-je).

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

API se avtenticira tako, da posredujete svoj [api key](https://fastcomments.com/auth/my-account/api-secret) bodisi kot glavo `X-API-KEY` ali kot parameter poizvedbe `API_KEY`. Za klice API potrebujete tudi svoj `tenantId`. Ta ga lahko pridobite na isti strani kot svoj api key.

### Varnostno obvestilo

Ti končni točki so namenjeni klicem iz **strežnika**. __NE__ jih kličite iz brskalnika. Če to storite, boste razkrili svoj API ključ — to bo kdorkoli, ki lahko vidi izvorno kodo strani, dobil popoln dostop do vašega računa!

#### Avtentikacijska možnost ena - Glave

- Glava: `X-API-KEY`
- Glava: `X-TENANT-ID`

#### Avtentikacijska možnost dva - Parametri poizvedbe

- Parameter poizvedbe: `API_KEY`
- Parameter poizvedbe: `tenantId`

### Branje lastnih zapisov

FastComments zagotavlja Active-Active razpoložljivost. Zahteve iz vašega podatkovnega centra so usmerjene na [najbližjo točko prisotnosti](https://sophon.fastcomments.com/) vaši. To je samodejno in običajno lahko opazite semantiko "preberi-svoj-zapis". Če želite biti prepričani, da boste prebrali svoje zapise, lahko svoje zahteve pripnete na določeno regijo tako, da za gostitelja API uporabite to regijo (vendar to za večino integracij običajno ni potrebno):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Upoštevajte, da boste morda želeli določiti rezervno možnost (fallback), saj smo v preteklosti označili nekatere vstopne točke kot zastarele in za preklop uporabljamo nova imena.