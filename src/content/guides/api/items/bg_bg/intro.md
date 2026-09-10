### FastComments API

FastComments предоставя API за взаимодействие с множество ресурси. Създавайте интеграции с нашата платформа или дори създавайте свои собствени клиенти!

В тази документация ще намерите всички поддържани ресурси от API, документирани с техните типове заявки и отговори.

За Enterprise клиенти, целият достъп до API се записва в одитния журнал.

### Генерирани SDK‑ове

FastComments сега генерира [API Spec](https://fastcomments.com/js/swagger.json) от нашия код (това все още не е завършено, но включва множество API-та).

Също така вече имаме SDK‑ове за популярни езици:

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

### Удостоверяване

API‑то се удостоверява, като предадете вашия [api key](https://fastcomments.com/auth/my-account/api-secret) като заглавие `X-API-KEY` или като параметър на заявката `API_KEY`. Също така ще ви е необходим вашият `tenantId` за извършване на API повиквания. Той може да се извлече от същата страница, където се намира вашият api key.

### Бележка за сигурност

Тези маршрути са предназначени да се извикват от **сървър**. __НЕ__ ги извиквайте от браузър. Ако го направите, вашият API ключ ще бъде изложен – това ще даде пълен достъп до вашия акаунт на всеки, който може да види изходния код на страницата!

#### Опция за удостоверяване 1 – Заглавки

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Опция за удостоверяване 2 – Параметри на заявката

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Опция за удостоверяване 3 – OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Приложения, които се свързват чрез [MCP server](https://docs.fastcomments.com/guide-llm-kit.html), получават токен чрез OAuth вместо API ключ. Този токен работи за всеки крайна точка тук. Наемателят е подразбиращ се от токена, така че `tenantId` е незадължителен, но трябва да съвпада с токена, ако е зададен. `GET` заявките изискват обхват `read`, а всички останали методи – обхват `write`. Откриването започва на `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Четене на собствените ви записи

FastComments предоставя активна‑активна наличност. Заявките от вашия център за данни се маршрутизират към [най-близката точка на присъствие](https://sophon.fastcomments.com/) до вашата. Това е автоматично и обикновено можете да наблюдавате семантиката „четене‑след‑запис“. Ако искате да сте сигурни, че четете собствените си записи, можете да фиксирате вашите заявки към определен регион, като използвате този регион като API хост (въпреки че обикновено това не е необходимо за повечето интеграции):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Имайте предвид, че ако направите това, може да искате да дефинирате резервен вариант, тъй като в миналото сме премахвали входни възли и използваме нови имена за превключването.