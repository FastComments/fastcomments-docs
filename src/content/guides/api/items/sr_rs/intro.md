### API платформе FastComments

FastComments обезбеђује API за интеракцију са многим ресурсима. Изградите интеграције са нашом платформом, или чак направите своје клијенте!

У овој документацији наћи ћете све ресурсе које API подржава, са документацијом њихових типова захтева и одговора.

За Enterprise кориснике, сав приступ API-ју се бележи у Audit Log.

### Generated SDKs

FastComments сада генерише [API Spec](https://fastcomments.com/js/swagger.json) из нашег кода (ово још није потпуно, али укључује многе API-је).

Такође сада имамо SDK-ове за популарне језике:

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

API се аутентификује прослеђивањем вашег [api key](https://fastcomments.com/auth/my-account/api-secret) као или `X-API-KEY` заглавља или као `API_KEY` query параметра. Такође ће вам требати ваш `tenantId` за позиве API-ја. Он се може преузети са исте странице као и ваш api key.

### Напомена о безбедности

Ове руте треба позивати са **сервера**. __НЕ ПОЗИВАЈТЕ__ их из прегледача. То би открило ваш API key - то ће омогућити пун приступ вашем налогу било коме ко може видети изворни код странице!

#### Authentication Option One - Headers

- Заглавље: `X-API-KEY`
- Заглавље: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Параметар упита: `API_KEY`
- Параметар упита: `tenantId`