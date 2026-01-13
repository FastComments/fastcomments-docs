### The FastComments API

FastComments предоставя API за взаимодействие с множество ресурси. Създавайте интеграции с нашата платформа или дори изграждайте свои собствени клиенти!

В тази документация ще намерите всички поддържани ресурси от API, документирани с техните типове заявки и отговори.

За корпоративни клиенти, целият достъп до API се записва в дневника на одита.

### Generated SDKs

FastComments вече генерира един [API Spec](https://fastcomments.com/js/swagger.json) от нашия код (това все още не е пълно, но включва много APIs).

Също така вече разполагаме със SDK за популярни езици:

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

### Authentication

API-то се удостоверява чрез подаване на вашия [api key](https://fastcomments.com/auth/my-account/api-secret) като или `X-API-KEY` хедър, или като `API_KEY` query параметър. Също така ще ви е необходим вашият `tenantId` за извършване на API повиквания. Той може да бъде получен от същата страница като вашия api key.

### Security Note

Тези рутове са предназначени да се вика от **сървър**. __НЕ__ ги викайте от браузър. Правенето на това ще изложи вашия API key - това ще предостави пълен достъп до вашия акаунт на всеки, който може да види изходния код на страница!

#### Authentication Option One - Headers

- Хедър: `X-API-KEY`
- Хедър: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Параметър на заявката: `API_KEY`
- Параметър на заявката: `tenantId`