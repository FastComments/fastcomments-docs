После настройки SAML в FastComments, вам нужно добавить FastComments в качестве Service Provider у вашого провайдера ідентифікації.

### Загальна конфігурація IdP

Більшість провайдерів ідентифікації вимагають наступну інформацію для додавання FastComments як SAML-додатка:

#### Обов'язкова інформація про Service Provider

Ці значення автоматично генеруються і відображаються на сторінці конфігурації SAML у FastComments:

**SP Entity ID / Audience**
- Формат: `https://fastcomments.com/saml/{your-tenant-id}`
- Це унікально ідентифікує ваш інстанс FastComments

**Assertion Consumer Service (ACS) URL**
- Формат: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Куди ваш IdP надсилає SAML-відповіді після автентифікації

**SP Metadata URL** *(якщо підтримується вашим IdP)*
- Формат: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Надає повну конфігурацію SAML у форматі XML

**SAML Login URL**
- Формат: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Пряме посилання для ініціації SAML-автентифікації

### Обов'язкові атрибути SAML

Налаштуйте ваш провайдер ідентифікації, щоб надсилати ці атрибути з SAML-відповідями:

#### Необхідні атрибути

**Email Address** *(обов'язково)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Призначення**: Унікальна ідентифікація користувача та повідомлення
- **Формат**: Дійсна електронна адреса

#### Додаткові атрибути

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Призначення**: Відображуване ім'я користувача

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Призначення**: Відображуване прізвище користувача

**Roles** *(важливо для контролю доступу)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Призначення**: Призначення ролей і дозволів у FastComments
- **Формат**: Масив рядків ролей або значення, розділені комами

### Поширені конфігурації провайдерів ідентифікації

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - Знайдіть "FastComments" або створіть власний SAML-додаток
   - Використайте інформацію SP, надану FastComments

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Використовуйте "Create New App" і виберіть SAML 2.0
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
   - Використайте URL метаданих FastComments або ручну конфігурацію
   - Налаштуйте інформацію SP як вказано

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership або кастомні claims

### Гнучкість імен атрибутів

FastComments приймає інформацію про ролі з кількох імен атрибутів, щоб врахувати різні конфігурації IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Ця гнучкість забезпечує сумісність з різними провайдерами ідентифікації без вимоги конкретних конвенцій іменування атрибутів.

### Тестування вашої конфігурації

Після налаштування провайдера ідентифікації:

1. Збережіть конфігурацію IdP
2. Протестуйте з виділеним тестовим обліковим записом користувача
3. Перевірте, що атрибути надсилаються правильно
4. Переконайтесь, що ролі правильно відображаються
5. Переконайтесь, що потік автентифікації успішно завершується

Більшість провайдерів ідентифікації пропонують інструменти тестування SAML для перевірки конфігурації перед розгортанням для користувачів у продакшн.