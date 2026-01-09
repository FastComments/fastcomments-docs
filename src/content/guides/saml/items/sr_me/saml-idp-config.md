Након конфигурисања SAML-а у FastComments, потребно је да подесите FastComments као Провајдера услуге у вашем провајдеру иденитета.

### Општа конфигурација IdP-а

Већина провајдера иденитета захтева следеће информације да би додали FastComments као SAML апликацију:

#### Обавезне информације о провајдеру услуге

Ове вредности се аутоматски генеришу и приказују на вашој FastComments SAML страници за конфигурацију:

**SP Entity ID / Audience**
- Формат: `https://fastcomments.com/saml/{your-tenant-id}`
- Ово јединствено идентификује вашу FastComments инстанцу

**Assertion Consumer Service (ACS) URL**
- Формат: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Где ваш IdP шаље SAML одговоре након аутентификације

**SP Metadata URL** *(ако ваш IdP подржава)*
- Формат: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Пружа полну SAML конфигурацију у XML формату

**SAML Login URL**
- Формат: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Директан линк за иницирање SAML аутентификације

### Потребни SAML атрибути

Подесите ваш провајдер иденитета да шаље ове атрибуте са SAML одговорима:

#### Основни атрибути

**Адреса електронске поште** *(обавезно)*
- **Име атрибута**: `email`, `emailAddress`, или `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Сврха**: Јединствена идентификација корисника и обавештења
- **Формат**: Важећа адреса електронске поште

#### Опционални атрибути

**Име**
- **Имена атрибута**: `firstName`, `givenName`, или `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Сврха**: Приказно име корисника

**Презиме**
- **Имена атрибута**: `lastName`, `surname`, или `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Сврха**: Приказно име корисника

**Улоге** *(Важно за контролу приступа)*
- **Имена атрибута**: `roles`, `groups`, `memberOf`, или прилагођена имена атрибута
- **Сврха**: Додавање улога и дозвола у FastComments-у
- **Формат**: Низ стрингова улога или вредности одвојене зарезом

### Честе конфигурације провајдера иденитета

#### Microsoft Azure AD

1. **Додајте Enterprise Application**
   - Претражите "FastComments" или креирајте прилагођену SAML апликацију
   - Користите SP информације које пружа FastComments

2. **Конфигуришите атрибуте**
   - Email: `user.mail` или `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` или групе у директоријуму

#### Okta

1. **Креирајте SAML апликацију**
   - Користите "Create New App" и изаберите SAML 2.0
   - Конфигуришите са FastComments SP информацијама

2. **Изјаве о атрибутима**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` или прилагођени атрибути

#### Google Workspace

1. **Додајте SAML апликацију**
   - Идите на Apps > Web and mobile apps > Add App > Add custom SAML app
   - Конфигуришите са FastComments SP информацијама

2. **Мапирање атрибута**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups или прилагођени атрибути

#### Active Directory Federation Services (ADFS)

1. **Додајте Relying Party Trust**
   - Користите FastComments metadata URL или ручну конфигурацију
   - Конфигуришите SP информације како су прућене

2. **Правила за изјаве (Claim Rules)**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Чланство у групи или прилагођене изјаве

### Флексибилност имена атрибута

FastComments прихвата информације о улогама из више имена атрибута како би се прилагодио различитим конфигурацијама IdP-а:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ова флексибилност обезбеђује компатибилност са разним провајдерима идентитета без потребе за специфичним конвенцијама именовања атрибута.

### Тестирање ваше конфигурације

Након конфигурације вашег провајдера иденитета:

1. Сачувајте IdP конфигурацију
2. Тестирајте са посебним тест корисничким налогом
3. Верификујте да ли се атрибути правилно шаљу
4. Проверите да ли су улоге правилно мапиране
5. Осигурајте да се аутентификациони флоу успешно завршава

Већина провајдера иденитета нуди SAML алате за тестирање како би верификовали конфигурацију пре примене на продукционе кориснике.