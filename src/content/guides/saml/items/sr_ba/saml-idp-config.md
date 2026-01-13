Након конфигурисања SAML-а у FastComments-у, потребно је поставити FastComments као Service Provider у вашем провајдеру идентитета.

### Општа конфигурација IdP-а

Већина провајдера идентитета захтијева следеће информације да би додала FastComments као SAML апликацију:

#### Обавезне информације о SP-у

Ове вриједности се аутоматски генеришу и приказују на вашој FastComments SAML страници за конфигурацију:

**SP Entity ID / Audience**
- Формат: `https://fastcomments.com/saml/{your-tenant-id}`
- Ово јединствено идентификује вашу FastComments инстанцу

**Assertion Consumer Service (ACS) URL**
- Формат: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Гдје ваш IdP шаље SAML одговоре након аутентификације

**SP Metadata URL** *(if supported by your IdP)*
- Формат: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Пружа комплетну SAML конфигурацију у XML формату

**SAML Login URL**
- Формат: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Директан линк за покретање SAML аутентификације

### Обавезни SAML атрибути

Конфигуришите вашег провајдера идентитета да шаље ове атрибуте са SAML одговорима:

#### Основни атрибути

**Адреса е-поште** *(Обавезно)*
- **Име атрибута**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Сврха**: Јединствена идентификација корисника и обавјештења
- **Формат**: Важећа адреса е-поште

#### Опционални атрибути

**Име**
- **Имена атрибута**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Сврха**: Приказно име корисника

**Презиме**
- **Имена атрибута**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Сврха**: Приказно име корисника

**Улоге** *(Важне за контролу приступа)*
- **Имена атрибута**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Сврха**: Додјела улога и дозвола у FastComments-у
- **Формат**: Низ стрингова који представљају улоге или вриједности раздвојене зарезом

### Уобичајене конфигурације провајдера идентитета

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - Тражите "FastComments" или креирајте прилагођену SAML апликацију
   - Користите SP информације које пружа FastComments

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Користите "Create New App" и изаберите SAML 2.0
   - Конфигуришите са FastComments SP информацијама

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Идите на Apps > Web and mobile apps > Add App > Add custom SAML app
   - Конфигуришите са FastComments SP информацијама

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - Користите FastComments metadata URL или ручну конфигурацију
   - Конфигуришите SP информације како је достављено

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Флексибилност имена атрибута

FastComments прихвата информације о улогама из више имена атрибута како би подржао различите конфигурације IdP-а:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ова флексибилност обезбјеђује компатибилност са различитим провајдерима идентитета без потребе за специфичним конвенцијама именовања атрибута.

### Тестирање ваше конфигурације

Након конфигурисања вашег провајдера идентитета:

1. Сачувајте IdP конфигурацију
2. Тестирајте са посебним тест корисничким налогом
3. Провјерите да ли се атрибути правилно шаљу
4. Провјерите да ли су улоге исправно мапиране
5. Осигурајте да се аутентификациони ток успјешно заврши

Већина провајдера идентитета нуди SAML алате за тестирање како би валидирали конфигурацију прије имплементације код продукционих корисника.