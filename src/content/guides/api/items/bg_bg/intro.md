### The FastComments API

FastComments предоставя API за взаимодействие с много ресурси. Изграждайте интеграции с нашата платформа или дори създавайте собствени клиенти!

В тази документация ще намерите всички поддържани ресурси от API, документирани с техните типове за заявки и отговори.

За Enterprise клиенти целият достъп до API се записва в Дневника за одит.

### Generated SDKs

FastComments вече генерира един [API Spec](https://fastcomments.com/js/swagger.json) от нашия код (това все още не е завършено, но включва много API-та).

Имаме също така SDK-та за популярни езици:

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

API-то се удостоверява чрез предаване на вашия [api key](https://fastcomments.com/auth/my-account/api-secret) като или `X-API-KEY` хедър, или като заявъчен параметър `API_KEY`. Също така ще ви е необходим вашият `tenantId` за извършване на API повиквания. Той може да бъде получен от същата страница като вашия api key.

### Security Note

Тези рути са предназначени да се извикват от **сървър**. __DO NOT__ ги извиквайте от браузър. Правенето на това ще изложи вашия API ключ - това ще предостави пълен достъп до вашия акаунт на всеки, който може да види изходния код на страницата!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Reading Your Own Writes

FastComments предоставя Active-Active наличност. Заявките от вашия център за данни се маршрутизират до [най-близката точка на присъствие](https://sophon.fastcomments.com/) спрямо вашата. Това е автоматично и обикновено можете да наблюдавате семантиката „четене след запис“. Ако искате да сте сигурни, че ще четете собствените си записи, можете да фиксирате заявките си към определен регион, като използвате този регион като неговия API хост (въпреки че това обикновено не е необходимо за повечето интеграции):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Забележка: ако направите това, може да искате да дефинирате резервен вариант, тъй като в миналото сме маркирали входни възли като остарели и сме използвали нови имена при превключване.