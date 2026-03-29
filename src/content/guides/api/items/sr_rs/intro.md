### FastComments API

FastComments обезбеђује API за рад са бројним ресурсима. Изградите интеграције са нашом платформом, или чак направите сопствене клијенте!

У овој документацији наћи ћете све ресурсе које API подржава, са документацијом о типовима захтева и одговора.

За Enterprise кориснике, сав приступ API-ју се бележи у Audit Log-у.

### Генерисани SDK-ови

FastComments сада генерише [API Spec](https://fastcomments.com/js/swagger.json) из нашег кода (ово још није потпуно, али обухвата многе API-је).

Такође имамо SDK-ове за популарне језике:

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

### Аутентикација

API се аутентикује прослеђивањем вашег [api key](https://fastcomments.com/auth/my-account/api-secret) као или `X-API-KEY` заглавља или `API_KEY` параметра у упиту. Такође ће вам бити потребан ваш `tenantId`
за позиве API-ја. То можете добити са исте странице као и ваш api key.

### Безбедносна напомена

Ове руте су намењене да се позивају са **сервера**. __НЕ ПОЗИВАТЕ__ их из претраживача. То ће открити ваш API key — ово ће обезбедити пун приступ вашем налогу
сваком ко може да види изворни код странице!

#### Опција аутентикације - Заглавља

- Заглавље: `X-API-KEY`
- Заглавље: `X-TENANT-ID`

#### Опција аутентикације - Параметри упита

- Параметар упита: `API_KEY`
- Параметар упита: `tenantId`

### Читање ваших сопствених уписа

FastComments пружа Active-Active доступност. Захтеви из вашег дата центра се усмеравају на [најближу тачку присуства](https://sophon.fastcomments.com/) у односу на вашу. Ово је аутоматско, и нормално можете посматрати семантику „čitaj-što-si-napisao“ (read-your-write). Ако желите да будете сигурни да ћете читати своје уписе, можете да закачите (pin) своје захтеве за одређену регију користећи ту регију као њен API host (међутим ово обично није потребно за већину интеграција):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Имајте на уму да ако то урадите можда ћете желети да дефинишете fallback, јер смо у прошлости застарели entrypoint ноде и користили нова имена за пребацивање.

---