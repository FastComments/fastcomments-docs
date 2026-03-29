### FastComments API

FastComments пружа API за интеракцију са бројним ресурсима. Направите интеграције са нашом платформом, или чак изградите своје клијенте!

У овој документацији наћи ћете све ресурсе које API подржава, документоване са типовима захтјева и одговора.

За Enterprise купце, сав приступ API-ју се бележи у дневнику ревизије.

### Генерисани SDK-ови

FastComments сада генерише [API спецификација](https://fastcomments.com/js/swagger.json) из нашег кода (ово још није у потпуности завршено, али укључује многе API-је).

Такођер сада имамо SDK-ове за популарне језике:

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

API се аутентификује прослеђивањем вашег [api key](https://fastcomments.com/auth/my-account/api-secret) као или `X-API-KEY` заглавље или `API_KEY` query параметар. Такођер ће вам требати ваш `tenantId`
за прављење API позива. Ово се може преузети са исте странице као и ваш api key.

### Напомена о безбједности

Ове руте су намјењене за позивање са **сервера**. __НЕ ПОВЛАЧИТЕ__ их из браузера. То ће открити ваш API key - ово ће пружити пун приступ вашем налогу
сваком ко може видјети исходни код странице!

#### Опција аутентификације једна - Заглавља

- Заглавље: `X-API-KEY`
- Заглавље: `X-TENANT-ID`

#### Опција аутентификације два - Параметри упита

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Читање сопствених уписа

FastComments обезбјеђује Active-Active доступност. Захтјеви из вашег дата центра усмјеравају се ка [најближоj тачки присуства](https://sophon.fastcomments.com/) у односу на ваш. Ово је аутоматско, и обично се обезбјеђује семантика да ћете моћи прочитати сопствене уписе. Ако желите бити сигурни да ћете читати своје уписе, можете фиксирати своје захтјеве на одређени регион користећи тај регион као API хост (међутим ово обично није потребно за већину интеграција):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Имајте на уму да, ако то урадите, можда ћете желети дефинисати резервну опцију (fallback), јер смо у прошлости застарили улазне чворове и користили нова имена за пребацивање.