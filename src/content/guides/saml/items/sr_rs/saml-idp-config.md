Након конфигурисања SAML-а у FastComments, потребно је да подесите FastComments као Service Provider у вашем провајдеру идентитета.

### Општа конфигурација IdP-а

Већина провајдера идентитета захтева следеће информације да би додали FastComments као SAML апликацију:

#### Обавезне информације о Service Provider-у

Вредности испод се аутоматски генеришу и приказују на вашој FastComments SAML страници за конфигурацију:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Ово једнозначно идентификује вашу FastComments инстанцу

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Где ваш IdP шаље SAML одговоре након аутентификације

**SP Metadata URL** *(ако ваш IdP то подржава)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Пружа комплетну SAML конфигурацију у XML формату

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Директан линк за покретање SAML аутентификације

### Потребни SAML атрибути

Конфигуришите вашег провајдера идентитета да шаље ове атрибуте са SAML одговорима:

#### Основни атрибути

**Email Address** *(Обавезно)*
- **Attribute Name**: `email`, `emailAddress`, или `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Сврха**: Јединствена идентификација корисника и обавештења
- **Формат**: Важећа адреса е-поште

#### Опционални атрибути

**First Name**
- **Attribute Names**: `firstName`, `givenName`, или `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Сврха**: Приказно име корисника

**Last Name**
- **Attribute Names**: `lastName`, `surname`, или `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Сврха**: Приказно име корисника

**Roles** *(Важно за контролу приступа)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, или прилагођена имена атрибута
- **Сврха**: Додела улога и дозвола у FastComments
- **Формат**: Низ стрингова који представљају улоге или вредности раздвојене зарезом

### Уобичајене конфигурације провајдера идентитета

#### Microsoft Azure AD

1. **Додајте Enterprise апликацију**
   - Претражите „FastComments“ или креирајте прилагођену SAML апликацију
   - Користите SP информације које пружа FastComments

2. **Конфигуришите атрибуте**
   - Email: `user.mail` или `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` или директоријумске групе

#### Okta

1. **Креирајте SAML апликацију**
   - Користите „Create New App“ и изаберите SAML 2.0
   - Конфигуришите користећи FastComments SP информације

2. **Изјаве о атрибутима (Attribute Statements)**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` или прилагођени атрибути

#### Google Workspace

1. **Додајте SAML апликацију**
   - Идите на Apps > Web and mobile apps > Add App > Add custom SAML app
   - Конфигуришите користећи FastComments SP информације

2. **Мапирање атрибута**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups или прилагођени атрибути

#### Active Directory Federation Services (ADFS)

1. **Додајте Relying Party Trust**
   - Користите FastComments metadata URL или ручну конфигурацију
   - Конфигуришите SP информације како је наведено

2. **Правила захтева (Claim Rules)**
   - Email: Email Address захтев
   - Name: Name ID захтев
   - Roles: Чланство у групи или прилагођени захтеви

### Флексибилност имена атрибута

FastComments прихвата информације о улогама из више имена атрибута како би се прилагодио различитим IdP конфигурацијама:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ова флексибилност обезбеђује компатибилност са различитим провајдерима идентитета без потребе за специфичним конвенцијама назива атрибута.

### Тестирање ваше конфигурације

Након конфигурисања вашег провајдера идентитета:

1. Сачувајте IdP конфигурацију
2. Тестирајте са посебним тест корисничким налогом
3. Проверите да ли се атрибути шаљу исправно
4. Проверите да ли су улоге правилно мапиране
5. Уверите се да процес аутентификације успешно завршава

Већина провајдера идентитета нуди SAML алате за тестирање како би се валидавала конфигурација пре имплементације код продукционих корисника.