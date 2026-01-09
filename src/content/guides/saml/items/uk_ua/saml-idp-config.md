Після налаштування SAML у FastComments потрібно додати FastComments як постачальника послуг у вашому провайдері ідентифікації.

### Загальна конфігурація IdP

Більшість провайдерів ідентифікації вимагають наступну інформацію для додавання FastComments як SAML-застосунку:

#### Необхідна інформація про Service Provider

Ці значення автоматично генеруються і відображаються на сторінці конфігурації SAML у FastComments:

**SP Entity ID / Audience**
- Формат: `https://fastcomments.com/saml/{your-tenant-id}`
- Це унікально ідентифікує ваш екземпляр FastComments

**Assertion Consumer Service (ACS) URL**
- Формат: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Куди ваш IdP надсилає SAML-відповіді після автентифікації

**SP Metadata URL** *(якщо підтримується вашим IdP)*
- Формат: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Надає повну конфігурацію SAML у форматі XML

**SAML Login URL**
- Формат: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Пряме посилання для ініціювання SAML-автентифікації

### Необхідні атрибути SAML

Налаштуйте ваш провайдер ідентифікації для відправки цих атрибутів у SAML-відповідях:

#### Обов'язкові атрибути

**Email Address** *(обов'язково)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Унікальна ідентифікація користувача та повідомлення
- **Format**: Дійсна електронна адреса

#### Додаткові атрибути

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: Відображуване ім'я користувача

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: Відображуване прізвище користувача

**Roles** *(Важливо для контролю доступу)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: Призначення ролей та прав у FastComments
- **Format**: Масив рядків ролей або значення, розділені комами

### Поширені конфігурації провайдерів ідентифікації

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - Шукайте "FastComments" або створіть власний SAML-застосунок
   - Використайте інформацію SP, надану FastComments

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Використайте "Create New App" і виберіть SAML 2.0
   - Налаштуйте за інформацією SP від FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Перейдіть до Apps > Web and mobile apps > Add App > Add custom SAML app
   - Налаштуйте за інформацією SP від FastComments

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - Використайте SP metadata URL або ручну конфігурацію
   - Налаштуйте інформацію SP, як надано

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Гнучкість назв атрибутів

FastComments приймає інформацію про ролі з кількох назв атрибутів, щоб підтримати різні конфігурації IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ця гнучкість забезпечує сумісність з різними провайдерами ідентифікації без вимоги конкретних найменувань атрибутів.

### Тестування вашої конфігурації

Після налаштування провайдера ідентифікації:

1. Збережіть конфігурацію IdP
2. Протестуйте за допомогою виділеного тестового облікового запису
3. Переконайтеся, що атрибути надсилаються правильно
4. Перевірте, що ролі відображаються коректно
5. Переконайтеся, що процес автентифікації завершується успішно

Більшість провайдерів ідентифікації пропонують інструменти тестування SAML для перевірки конфігурації перед розгортанням для користувачів у production.