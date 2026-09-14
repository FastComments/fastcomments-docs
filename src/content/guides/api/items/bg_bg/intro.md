### FastComments API

FastComments предоставя API за взаимодействие с множество ресурси. Създавайте интеграции с нашата платформа или дори създавайте свои собствени клиенти!

В тази документация ще намерите всички поддържани ресурси от API-то, документирани с техните типове заявки и отговори.

За Enterprise клиентите, целият достъп до API е записван в одитния журнал.

### Генерирани SDK

FastComments сега генерира [API спецификация](https://fastcomments.com/js/swagger.json) от нашия код (все още не е пълна, но включва много API-та).

Също така вече имаме SDK за популярни езици:

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

API се удостоверява, като предадете вашия [api key](https://fastcomments.com/auth/my-account/api-secret) като заглавие `X-API-KEY` или като параметър на заявката `API_KEY`. Също така ще ви е необходим вашият `tenantId` за извършване на API повиквания. Той може да се извлече от същата страница като вашия API ключ.

### Забележка за сигурността

Тези маршрути са предназначени да се извикват от **сървър**. __НЕ ПРИЗОВЯВАЙТЕ__ ги от браузър. Това ще разкрие вашия API ключ – ще даде пълен достъп до вашия акаунт на всеки, който може да види изходния код на страницата!

#### Опция за удостоверяване 1 – Заглавки

- Заглавка: `X-API-KEY`
- Заглавка: `X-TENANT-ID`

#### Опция за удостоверяване 2 – Параметри на заявката

- Параметър на заявката: `API_KEY`
- Параметър на заявката: `tenantId`

#### Опция за удостоверяване 3 – OAuth Bearer Token

- Заглавка: `Authorization: Bearer fcat_...`

Приложения от трети страни, като Zapier и клиенти на [MCP сървър](https://docs.fastcomments.com/guide-llm-kit.html) получават токен чрез OAuth вместо API ключ. Този токен работи за всяка крайна точка тук. Наемателят (tenant) е подразбиращ се от токена, така че `tenantId` е незадължителен, но трябва да съвпада с токена, ако е зададен. `GET` заявките изискват обхват `read`, а всички останали методи – обхват `write`. Пълният процес, включително регистрация на клиент, PKCE, обновяване и отмяна, е документиран под [OAuth Authorization](#oauth). Откриването започва на `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Четене на собствените ви записи

FastComments осигурява Active-Active наличност. Заявките от вашия датацентър се маршрутизират към най-близката точка на присъствие за вас. Това е автоматично и обикновено можете да наблюдавате семантиката „прочети‑твоето‑писане“. Ако искате да сте сигурни, че четете собствените си записи, можете да фиксирате вашите заявки към определен регион, като използвате този регион като API хост (въпреки че обикновено това не е необходимо за повечето интеграции):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Имайте предвид, че ако направите това, може да искате да дефинирате резервен вариант, тъй като в миналото сме премахвали входни възли и използваме нови имена за превключването.