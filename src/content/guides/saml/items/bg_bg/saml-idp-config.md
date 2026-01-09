След като конфигурирате SAML в FastComments, трябва да настроите FastComments като Доставчик на услуги в вашия доставчик на идентичност.

### Общи настройки на IdP

Повечето доставчици на идентичност изискват следната информация, за да добавят FastComments като SAML приложение:

#### Задължителна информация за доставчика на услуги

Тези стойности се генерират автоматично и се показват на страницата за SAML конфигурация на FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Това уникално идентифицира вашия FastComments инстанс

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Към къде вашият IdP изпраща SAML отговорите след удостоверяване

**SP Metadata URL** *(ако се поддържа от вашия IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Предоставя пълната SAML конфигурация във формат XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Пряк линк за иницииране на SAML удостоверяване

### Задължителни SAML атрибути

Конфигурирайте вашия доставчик на идентичност да изпраща тези атрибути с SAML отговорите:

#### Основни атрибути

**Email Address** *(Задължително)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Уникална идентификация на потребителя и известия
- **Format**: Валиден имейл адрес

#### Незадължителни атрибути

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: Показвано име на потребителя

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: Показвано име на потребителя

**Roles** *(Важно за контрола на достъпа)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: Присвояване на роли и права във FastComments
- **Format**: Масив от низове с роли или стойности, разделени със запетаи

### Често срещани конфигурации за доставчици на идентичност

#### Microsoft Azure AD

1. **Добавяне на Enterprise Application**
   - Потърсете "FastComments" или създайте персонализирано SAML приложение
   - Използвайте информацията за SP, предоставена от FastComments

2. **Конфигуриране на атрибутите**
   - Email: `user.mail` или `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` или директория групи

#### Okta

1. **Създаване на SAML приложение**
   - Използвайте "Create New App" и изберете SAML 2.0
   - Конфигурирайте с SP информацията на FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` или персонализирани атрибути

#### Google Workspace

1. **Добавяне на SAML приложение**
   - Отидете в Apps > Web and mobile apps > Add App > Add custom SAML app
   - Конфигурирайте с SP информацията на FastComments

2. **Съпоставяне на атрибути**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups или персонализирани атрибути

#### Active Directory Federation Services (ADFS)

1. **Добавяне на Relying Party Trust**
   - Използвайте FastComments metadata URL или ръчна конфигурация
   - Конфигурирайте SP информацията както е предоставена

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership или персонализирани claim-и

### Гъвкавост на имената на атрибутите

FastComments приема информация за роли от множество имена на атрибути, за да се адаптира към различни конфигурации на IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Тази гъвкавост осигурява съвместимост с различни доставчици на идентичност, без да изисква специфични конвенции за именуване на атрибутите.

### Тестване на вашата конфигурация

След като конфигурирате вашия доставчик на идентичност:

1. Запазете конфигурацията на IdP
2. Тествайте с отделен тестов потребителски акаунт
3. Проверете дали атрибутите се изпращат правилно
4. Уверете се, че ролите са правилно съпоставени
5. Проверете, че потока на удостоверяване завършва успешно

Повечето доставчици на идентичност предлагат SAML инструменти за тестване, за да валидират конфигурацията преди внедряване за производствени потребители.