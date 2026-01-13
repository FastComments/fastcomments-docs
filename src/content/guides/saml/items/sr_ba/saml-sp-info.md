When SAML је омогућен у FastComments, систем аутоматски генерише информације о Provajderu usluge (SP) које су вам потребне за конфигурисање у вашем провајдеру идентитета.

### Pristup informacijama o Provajderu usluge

Informacije o SP-у се приказују на страници за SAML конфигурацију након омогућавања SAML аутентикације. Ове информације укључују све детаље које ваш провајдер идентитета треба да успостави SAML повјерљиви однос.

### Krajnje tačke Provajdera usluge

#### SP Entity ID / Audience
**Svrha**: Једнозначно идентификује вашу FastComments инстанцу као provajdera usluge  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Upotreba**: Конфигуришите ово као Entity ID или Audience у вашем IdP-у

Овај идентификатор осигурава да су SAML одговори намењени вашем специфичном FastComments tenant-у и спречава да SAML одговори буду прихваћени од стране других инстанци.

#### Assertion Consumer Service (ACS) URL
**Svrha**: Krajnja tačka на коју ваш IdP шаље SAML одговоре након аутентификације корисника  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Upotreba**: Конфигуришите ово као ACS URL или Reply URL у вашем IdP-у

Овде се корисници преусмеравају након успјешне аутентикације код провајдера идентитета, заједно са SAML асерцијом која садржи информације о кориснику.

#### SP Metadata URL
**Svrha**: Обезбјеђује комплетну SAML конфигурацију у стандардном XML формату  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Upotreba**: Неки IdP-ови могу аутоматски импортовати конфигурацију користећи овај URL

Metadata URL садржи све неопходне SP информације у XML формату, што омогућава лако аутоматско конфигурисање компатибилних провајдера идентитета.

#### SAML Login URL
**Svrha**: Директан линк за покретање SAML аутентикације за ваш tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Upotreba**: Повеже кориснике директно на SAML аутентикацију или за тестирање тока

Можете користити овај URL за тестирање SAML аутентикације или да омогућите корисницима директан линк за пријаву преко SAML-а.

### Podrška za SAML Binding-e

FastComments подржава следеће SAML binding-e:

#### HTTP-POST Binding
- **Primarni metod**: Најчешћи binding за SAML одговоре  
- **Sigurnost**: SAML одговор се шаље преко HTTP POST-а до ACS URL-а  
- **Upotreba**: Препоручује се за продукционе деплојменте

#### HTTP-Redirect Binding
- **Alternativni metod**: SAML одговор се шаље преко HTTP редиректа  
- **Ograničenja**: Ограничена величина payload-а због ограничења дужине URL-а  
- **Upotreba**: Подржано, али HTTP-POST је пожељан

### Name ID политика

FastComments конфигурише следећу Name ID политику у SAML захтјевима:

- **Подразумевани формат**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Алтернативни формати**: Persistent, Transient, Unspecified (конфигурисано)  
- **Потреба**: Адреса е-поште се користи као примарни идентификатор корисника

### Atributi SAML захтјева

При покретању SAML аутентикације, FastComments шаље захтјеве са овим карактеристикама:

#### Potpisivanje zahtjeva
- **Статус**: Опционо (конфигурисано)  
- **Алгоритам**: Одговара конфигурисаном алгоритму за потпис  
- **Сертификат**: Користи сертификат специфичан за tenant ако је потписивање захтева омогућено

#### Traženi atributi
FastComments тражи следеће атрибуте у SAML AuthnRequests:

- **Email**: Обавезно за идентификацију корисника  
- **First Name**: Опционо за приказ  
- **Last Name**: Опционо за приказ  
- **Roles/Groups**: Опционо за контролу приступа и дозволе

### Kopiranje informacija o SP-u

Страница за SAML конфигурацију обезбјеђује поља на која се може кликнути и која аутоматски копирају SP информације у ваш clipboard:

1. Кликните било које поље са информацијом о SP-у (Entity ID, ACS URL, итд.)  
2. Вриједност се аутоматски копира у ваш clipboard  
3. Залепите вриједност у конфигурацију вашег провајдера идентитета  
4. Кратко истакнуће указује на успјешно копирање

Ово олакшава прецизно пребацивање SP информација у ваш IdP без грешака при куцању.

### Informacije o SP sertifikatu

#### Upotreba sertifikata
- **Svrha**: Шифрује комуникацију и верификује идентитет SP-а  
- **Rotacija**: Сертификати се аутоматски управљају од стране FastComments  
- **Pristup**: Јавни сертификати су доступни преко metadata URL-а

#### Detalji sertifikata
- **Алгоритам**: RSA-2048 или виши  
- **Valjanost**: Сертификати се аутоматски обнављају прије истека  
- **Distribucija**: Доступно кроз стандардни SAML metadata

### Rješavanje problema sa SP konfiguracijom

Ако ваш провајдер идентитета пријави проблеме са SP информацијама:

1. **Провјерите URL-ове**: Осигурајте да сви URL-ови користе HTTPS и садрже исправан tenant ID  
2. **Провјерите metadata**: Користите metadata URL да верификујете конфигурацију  
3. **Тестирајте повезивост**: Осигурајте да ваш IdP може досећи FastComments крајње тачке  
4. **Валидирајте формат**: Потврдите да ваш IdP подржава формат информација о SP-у

Чести проблеми укључују:
- Неправилан tenant ID у URL-овима  
- Проблеми са мрежном повезаношћу између IdP-а и FastComments-а  
- IdP очекује другачије формате URL-ова или додатне опције конфигурације