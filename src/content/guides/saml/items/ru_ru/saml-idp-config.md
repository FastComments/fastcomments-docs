После настройки SAML в FastComments вам нужно настроить FastComments как поставщика услуг (Service Provider, SP) в вашем поставщике удостоверений (IdP).

### Общая конфигурация поставщика удостоверений (IdP)

Большинство поставщиков удостоверений требуют следующую информацию для добавления FastComments как SAML-приложения:

#### Необходимая информация о поставщике услуг

Эти значения автоматически генерируются и отображаются на странице конфигурации SAML в FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Это однозначно идентифицирует ваш экземпляр FastComments

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Куда ваш IdP отправляет SAML-ответы после аутентификации

**SP Metadata URL** *(если поддерживается вашим IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Предоставляет полную конфигурацию SAML в формате XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Прямая ссылка для инициирования SAML-аутентификации

### Обязательные атрибуты SAML

Настройте ваш поставщик удостоверений на отправку этих атрибутов в SAML-ответах:

#### Основные атрибуты

**Адрес электронной почты** *(обязательно)*
- **Имя атрибута**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Назначение**: уникальная идентификация пользователя и уведомления
- **Формат**: корректный адрес электронной почты

#### Необязательные атрибуты

**Имя**
- **Имена атрибутов**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Назначение**: отображаемое имя пользователя

**Фамилия**
- **Имена атрибутов**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Назначение**: отображаемое имя пользователя

**Роли** *(важно для управления доступом)*
- **Имена атрибутов**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Назначение**: назначение ролей и прав в FastComments
- **Формат**: массив строк ролей или значения, разделенные запятыми

### Типовые конфигурации поставщиков удостоверений

#### Microsoft Azure AD

1. **Добавить корпоративное приложение**
   - Найдите "FastComments" или создайте пользовательское SAML-приложение
   - Используйте информацию SP, предоставленную FastComments

2. **Настроить атрибуты**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Используйте "Create New App" и выберите SAML 2.0
   - Настройте с информацией SP FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Перейдите в Apps > Web and mobile apps > Add App > Add custom SAML app
   - Настройте с информацией SP FastComments

2. **Attribute Mapping**
   - Email: основной адрес электронной почты
   - First Name: имя
   - Last Name: фамилия
   - Roles: группы или пользовательские атрибуты

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - Используйте URL метаданных FastComments или настройку вручную
   - Настройте информацию SP согласно предоставленным данным

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: членство в группе или пользовательские утверждения

### Гибкость имен атрибутов

FastComments принимает информацию о ролях из нескольких имен атрибутов, чтобы учитывать разные конфигурации IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Такая гибкость обеспечивает совместимость с различными поставщиками удостоверений без требования конкретных соглашений об именовании атрибутов.

### Тестирование вашей конфигурации

После настройки вашего поставщика удостоверений:

1. Сохраните конфигурацию IdP
2. Протестируйте с выделённой тестовой учетной записью пользователя
3. Убедитесь, что атрибуты отправляются корректно
4. Проверьте, что роли правильно сопоставлены
5. Убедитесь, что процесс аутентификации завершается успешно

Большинство поставщиков удостоверений предлагают инструменты тестирования SAML для проверки конфигурации перед развертыванием для боевых пользователей.