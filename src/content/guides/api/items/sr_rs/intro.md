---
### FastComments API

FastComments обезбеђује API за интеракцију са многим ресурсима. Изградите интеграције са нашом платформом, или чак направите сопствене клијенте!

У овој документацији пронаћи ћете све подржане ресурсе API‑ја документоване са њиховим типовима захтева и одговора.

За Enterprise клијенте, сав приступ API‑ју се бележи у Аудит лог.

### Генерисани SDK‑ови

FastComments сада генерише [API Spec](https://fastcomments.com/js/swagger.json) из нашег кода (ово још није потпуно, али укључује многе API‑је).

Такође сада имамо SDK‑ове за популарне језике:

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

### Аутентификација

API се аутентификује прослеђивањем вашег [api кључа](https://fastcomments.com/auth/my-account/api-secret) као `X-API-KEY` заглавља или `API_KEY` параметра упита. Такође ће вам бити потребан ваш `tenantId` за извршавање API позива. Овај се може добити са исте странице као ваш api кључ.

### Безбедносна напомена

Ови путеви су намењени позиву са **сервера**. __НЕ ПРИЗИВАЈТЕ__ их из прегледача. То ће изложити ваш API кључ – што ће омогућити пун приступ вашем налогу свакоме ко може видети изворни код странице!

#### Опција аутентификације 1 – Заглавља

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Опција аутентификације 2 – Параметри упита

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Опција аутентификације 3 – OAuth Bearer токен

- Header: `Authorization: Bearer fcat_...`

Апликације које се повезују преко [MCP сервера](https://docs.fastcomments.com/guide-llm-kit.html) добијају токен преко OAuth уместо API кључа. Тај токен функционише на свим крајњим тачкама овде. Тенант је подразумеван токеном, тако да је `tenantId` опционо, али мора одговарати токену ако је наведен. `GET` захтеви захтевају `read` опсег, а сви остали методи захтевају `write` опсег. Откривање почиње на `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Читање сопствених уписа

FastComments обезбеђује Active-Active доступност. Захтеви из вашег датацентра се усмеравају ка [најближој тачки присутности](https://sophon.fastcomments.com/) у односу на ваш. Ово је аутоматско, и обично можете приметити семантику читај‑пиши‑са‑себом. Ако желите да будете сигурни да читате своје уписе, можете фиксирати захтеве на одређени регион користећи тај регион као API хост (међутим, ово обично није потребно за већину интеграција):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Имајте на уму да ако ово урадите, можда ћете желети да дефинишете резерву, јер смо у прошлости укинули улазне чворове и користимо нове називе за пребацивање.

---