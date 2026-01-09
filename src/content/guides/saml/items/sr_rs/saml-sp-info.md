When SAML је омогућен у FastComments, систем аутоматски генерише Service Provider (SP) информације које треба да конфигуришете у вашем identity provider-у.

### Приступ Service Provider информацијама

SP информације се приказују на вашој SAML страници за конфигурацију након укључивања SAML аутентификације. Ове информације садрже све детаље који су потребни вашем identity provider-у за успостављање SAML trust везе.

### Service Provider крајње тачке

#### SP Entity ID / Audience
**Purpose**: Једнозначно идентификује вашу FastComments инстанцу као Service Provider  
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Usage**: Конфигуришите ово као Entity ID или Audience у вашем IdP

Овај идентификатор осигурава да су SAML одговори намењени вашој конкретној FastComments tenant инстанци и спречава прихватање SAML одговора од других инстанци.

#### Assertion Consumer Service (ACS) URL
**Purpose**: Крајња тачка на коју ваш IdP шаље SAML одговоре након аутентификације корисника  
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Usage**: Конфигуришите ово као ACS URL или Reply URL у вашем IdP

Овде се корисници преусмеравају након успешне аутентификације код вашег identity provider-а, заједно са SAML тврђењем које садржи информације о кориснику.

#### SP Metadata URL
**Purpose**: Пружа потпуну SAML конфигурацију у стандардном XML формату  
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Usage**: Неки IdP-ови могу аутоматски увозити конфигурацију користећи овај URL

Metadata URL садржи све неопходне SP информације у XML формату, што олакшава аутоматску конфигурацију компатибилних identity provider-а.

#### SAML Login URL
**Purpose**: Директан линк за иницирање SAML аутентификације за ваш tenant  
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Usage**: Повеžite кориснике директно са SAML аутентификацијом или тестирајте ток

Можете користити овај URL да тестирате SAML аутентификацију или да корисницима пружите директан линк за пријаву преко SAML-а.

### SAML Binding подршка

FastComments подржава следеће SAML binding методе:

#### HTTP-POST Binding
- **Primary Method**: Најчешћа binding метода за SAML одговоре  
- **Security**: SAML одговор се шаље преко HTTP POST на ACS URL  
- **Usage**: Препоручено за продукционе инсталације

#### HTTP-Redirect Binding
- **Alternative Method**: SAML одговор се шаље преко HTTP redirect  
- **Limitations**: Ограничен капацитет payload-а због ограничења дужине URL-а  
- **Usage**: Подржано, али HTTP-POST је пожељнији

### Name ID Policy

FastComments конфигурише следећу Name ID политику у SAML захтевима:

- **Default Format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Alternative Formats**: Persistent, Transient, Unspecified (конфигурисано)  
- **Requirement**: Email адреса се користи као примарни идентификатор корисника

### SAML Request Attributes

При иницирању SAML аутентификације, FastComments шаље захтеве са следећим карактеристикама:

#### Request Signing
- **Status**: Опционално (конфигурисано)  
- **Algorithm**: Одговара конфигурисаном алгоритму потписа  
- **Certificate**: Користи tenant-специфичан сертификат ако је request signing омогућен

#### Requested Attributes
FastComments захтева следеће атрибуте у SAML AuthnRequests:

- **Email**: Захтевано за идентификацију корисника  
- **First Name**: Опционо за приказ  
- **Last Name**: Опционо за приказ  
- **Roles/Groups**: Опционо за контролу приступа и дозволе

### Копирање SP информација

SAML страница за конфигурацију пружа кликабельна поља која аутоматски копирају SP информације у ваш clipboard:

1. Кликните било које SP информацијско поље (Entity ID, ACS URL, итд.)  
2. Вредност се аутоматски копира у clipboard  
3. Налепите вредност у конфигурацију вашег identity provider-а  
4. Кратко истакнуће указује на успешно копирање

Ово олакшава тачан пренос SP информација у ваш IdP без грешака при куцању.

### SP Certificate информације

#### Коришћење сертификата
- **Purpose**: Шифрује комуникације и верификује SP идентитет  
- **Rotation**: Сертификати се аутоматски управљају од стране FastComments  
- **Access**: Jавни сертификати су доступни преко metadata URL

#### Детаљи о сертификату
- **Algorithm**: RSA-2048 или виши  
- **Validity**: Сертификати се аутоматски обнављају пре истека рока  
- **Distribution**: Доступни кроз стандардни SAML metadata

### Решавање проблема са SP конфигурацијом

Ако ваш identity provider пријави проблеме са SP информацијама:

1. **Verify URLs**: Потврдите да свака URL користи HTTPS и садржи правилан tenant ID  
2. **Check Metadata**: Користите metadata URL да проверите конфигурацију  
3. **Test Connectivity**: Потврдите да ваш IdP може да приступи FastComments крајњим тачкама  
4. **Validate Format**: Потврдите да ваш IdP подржава формат SP информација

Уобичајени проблеми укључују:
- Неисправан tenant ID у URL-овима  
- Проблеми са мрежном повезивошћу између IdP-а и FastComments-а  
- IdP очекује другачији формат URL-ова или додатне опције конфигурације