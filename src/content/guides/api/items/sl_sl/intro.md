### FastComments API

FastComments ponuja API za interakcijo s številnimi viri. Zgradite integracije z našo platformo ali celo ustvarite svoje odjemalce!

V tej dokumentaciji boste našli vse podprte vire API-ja, dokumentirane z njihovimi tipi zahtevkov in odgovorov.

Za podjetniške stranke je ves dostop do API-ja zabeležen v revizijskem dnevniku.

### Generirani SDK-ji

FastComments zdaj generira [API specifikacijo](https://fastcomments.com/js/swagger.json) iz naše kode (to še ni popolno, vendar vključuje številne API-je).

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

API je avtenticiran z posredovanjem vašega [api ključa](https://fastcomments.com/auth/my-account/api-secret) kot glave `X-API-KEY` ali kot poizvedbeni parameter `API_KEY`. Prav tako boste potrebovali vaš `tenantId` za izvajanje API klicev. Ta je mogoče pridobiti na isti strani kot vaš api ključ.

### Varnostno opombo

Ti naslovi so namenjeni klicanju s **strežnika**. __NE__ jih kličite iz brskalnika. To bo razkrilo vaš API ključ – kar bo omogočilo popoln dostop do vašega računa vsakomur, ki lahko vidi izvorno kodo strani!

#### Možnost avtentikacije ena – Glave

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Možnost avtentikacije dva – Poizvedbeni parametri

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Možnost avtentikacije tri – OAuth Bearer žeton

- Header: `Authorization: Bearer fcat_...`

Aplikacije tretjih oseb, kot sta Zapier in odjemalci [MCP strežnika](https://docs.fastcomments.com/guide-llm-kit.html), pridobijo žeton prek OAuth namesto API ključa. Ta žeton deluje na vseh končnih točkah tukaj. Najemnik je iz žetona izpeljan, zato je `tenantId` neobvezen, vendar se mora ujemati z žetonom, če je podan. `GET` zahteve potrebujejo obseg `read`, vsaka druga metoda pa obseg `write`. Celoten potek, vključno z registracijo odjemalca, PKCE, osveževanjem in preklicem, je dokumentiran pod [OAuth Authorization](#oauth). Odkrivanje se začne na `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Branje lastnih zapisov

FastComments zagotavlja Active-Active razpoložljivost. Zahteve iz vašega podatkovnega centra se usmerijo na [najbližjo točko prisotnosti](https://sophon.fastcomments.com/) do vas. To je samodejno in običajno lahko opazujete semantiko branja po pisanju. Če želite biti prepričani, da preberete svoje zapise, lahko svoje zahteve pripnete na določeno regijo tako, da uporabite to regijo kot svoj API gostitelj (vendar to običajno ni potrebno za večino integracij):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Upoštevajte, da če to storite, boste morda želeli določiti nadomestno možnost, saj smo v preteklosti opustili vstopne vozlišča in uporabljamo nova imena za preklop.