### FastComments API

FastComments пружа API за интеракцију са многим ресурсима. Изградите интеграције са нашом платформом, или чак направите своје клијенте!

У овој документацији наћи ћете све подржане ресурсе које API документује заједно са њиховим типовима захтева и одговора.

За Enterprise кориснике, сав приступ API-ју се бележи у Audit лог-у.

### Генерисани SDK-ови

FastComments сада генерише [API Spec](https://fastcomments.com/js/swagger.json) из нашег кода (ово још увек није потпуно, али укључује многе API-је).

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

API се аутентификује прослеђивањем вашег [api key](https://fastcomments.com/auth/my-account/api-secret) као или `X-API-KEY` заглавља или `API_KEY` query параметра. Такође ће вам требати ваш `tenantId` за прављење API позива. Ово се може преузети са исте странице као и ваш api key.

### Безбедносна напомена

Ове руте су намењене да се позивају са **сервера**. __НЕ ПОЗИВАТЕ__ их из претраживача. То ће открити ваш API кључ — то ће обезбедити пун приступ вашем налогу сваком ко може да види изворни код странице!

#### Authentication Option One - Headers

- Заглавље: `X-API-KEY`
- Заглавље: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Параметар упита: `API_KEY`
- Параметар упита: `tenantId`

### Читање сопствених уписа

FastComments обезбеђује Active-Active доступност. Захтеви са вашег дата-центра се усмеравају на [најближу тачку присуства](https://sophon.fastcomments.com/) у односу на вашу. Ово је аутоматско, и обично можете уочити семантику читања-ваших-записа. Ако желите да будете сигурни да читате своје сопствене уписе, можете фиксирати своје захтеве на одређену регију коришћењем те регије као њеног API хоста (међутим, ово обично није потребно за већину интеграција):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Имајте у виду да ако то урадите можда желите да дефинишете fallback, јер смо у прошлости декласификовали entrypoint чворове и користили нова имена за преусмеравање.