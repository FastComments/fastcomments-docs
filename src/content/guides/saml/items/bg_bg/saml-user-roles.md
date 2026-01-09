FastComments картографира SAML потребителски роли към вътрешни разрешения, позволявайки контрол на достъпа базиран на роли за вашата организация.

### FastComments Role System

FastComments използва система за разрешения, базирана на роли, при която потребителите могат да имат една или повече роли, които определят нивата им на достъп и възможностите им.

### Available FastComments Roles

#### Administrative Roles

**fc-account-owner**
- **Permissions**: Пълен административен достъп
- **Capabilities**: Всички функции, управление на фактури и плащания, управление на потребители
- **Use Case**: Основни администратори на акаунта и собственици

**fc-admin-admin**  
- **Permissions**: Административен достъп до повечето функции
- **Capabilities**: Управление на потребители, конфигурация, модериране. **Can administer other admins.**
- **Use Case**: Второстепенни администратори и ИТ персонал

**fc-billing-admin**
- **Permissions**: Управление на фактуриране и абонаменти
- **Capabilities**: Начини на плащане, фактури, промени в абонамента
- **Use Case**: Членове на финансовия екип и контакти за фактуриране

#### Specialized Roles

**fc-analytics-admin**
- **Permissions**: Достъп до аналитика и отчети
- **Capabilities**: Преглед на статистики за сайта, данни за ангажираност на потребителите
- **Use Case**: Маркетингови екипи и анализатори на данни

**fc-api-admin**
- **Permissions**: Достъп и управление на API
- **Capabilities**: API креденшъли, конфигурация на уебхукове
- **Use Case**: Разработчици и технически интегратори

**fc-moderator**
- **Permissions**: Възможности за модериране на коментари
- **Capabilities**: Одобряване/отхвърляне на коментари, управление на спам
- **Use Case**: Модератори на общността и мениджъри на съдържание

### Role Mapping Configuration

#### SAML Attribute Sources

FastComments приема информация за роли от различни имена на SAML атрибути, за да осигури съвместимост с различни доставчици на идентичност:

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Role Format Support

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Identity Provider Role Configuration

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - Дефинирайте FastComments роли във вашето приложение в Azure AD
   - Присвойте потребителите към съответните роли на приложението
   - Конфигурирайте claims да включват присвоените роли

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - Създайте групи, съвпадащи с имената на роли в FastComments
   - Присвойте потребителите към съответните групи
   - Конфигурирайте атрибутните заявления

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - Създайте организационни единици или групи
   - Наименувайте групите с префикси за роли на FastComments
   - Конфигурирайте съпоставянето на атрибути

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Default User Behavior

#### Users Without Roles

Когато SAML потребител няма роли или има неразпознати роли:
- Потребителят се създава като стандартен коментатор
- Не се предоставя административен достъп
- Може да публикува и управлява собствените си коментари
- Не може да достъпи функции на административния панел

#### Role Inheritance

- Потребителите могат да имат едновременно множество роли
- Разрешенията са кумулативни (опрделя се най-високото ниво на разрешение)
- Промените в ролите в IdP се отразяват при следващото влизане

### Managing SAML Users

#### User Creation

Когато потребител влезе чрез SAML за първи път:
1. **User Account**: Автоматично се създава акаунт с имейла като идентификатор
2. **Role Assignment**: Ролите се прилагат въз основа на SAML атрибутите
3. **Profile Information**: Името и фамилията се попълват, ако са предоставени
4. **Permission Activation**: Ролите стават активни веднага

#### Role Updates

Съществуващите SAML потребители получават актуализации на ролите:
1. **Login Trigger**: Актуализациите на роли се извършват при всяко SAML влизане
2. **Immediate Effect**: Новите разрешения се прилагат веднага
3. **Role Removal**: Премахнатите роли се отнемат автоматично
4. **Audit Trail**: Промените в ролите се записват в журналите на одита

### Custom Role Mapping

#### Enterprise Customization

За корпоративни клиенти със специфични изисквания:
- Персонализирани имена на роли могат да бъдат съпоставени с разрешенията на FastComments
- Могат да се реализират сложни йерархии на роли
- Може да се конфигурират контролите за достъп специфични за отделите

Свържете се с екипа за поддръжка на FastComments за конфигурации на персонализирано съпоставяне на роли.

#### Role Validation

FastComments валидира входящите роли:
- Неразпознатите роли се игнорират (не се отхвърлят)
- Грешно форматираните атрибути за роли се регистрират за отстраняване на проблеми
- Потребителите запазват съществуващите роли, ако SAML assertion не съдържа информация за роли

### Best Practices

#### Role Management

1. **Principle of Least Privilege**: Присвоявайте минимално необходимите разрешения
2. **Regular Auditing**: Преглеждайте ролите на потребителите и достъпа периодично  
3. **Clear Naming**: Използвайте описателни имена на групи във вашия IdP
4. **Documentation**: Поддържайте документация за присвояванията на роли

#### Security Considerations

1. **Role Attributes**: Уверете се, че атрибутите за роли са правилно защитени в SAML отговорите
2. **Attribute Validation**: Проверявайте, че само упълномощени системи могат да присвояват роли
3. **Access Reviews**: Редовно преглеждайте присвояванията на административни роли
4. **Monitoring**: Наблюдавайте промените в ролите и административните действия

### Troubleshooting Role Issues

#### Common Problems

**Roles Not Applied**:
- Проверете дали имената на SAML атрибутите съвпадат с поддържаните формати
- Уверете се, че IdP изпраща информация за роли
- Потвърдете, че стойностите на ролите съвпадат точно с имената на роли в FastComments

**Access Denied**:
- Проверете дали потребителят има подходяща роля, присвоена в IdP
- Проверете правилното писане и чувствителността към главни/малки букви на ролята
- Потвърдете, че ролята е правилно форматирана в SAML отговора

**Missing Permissions**:
- Прегледайте дефинициите на ролите и изискваните разрешения
- Проверете за конфликтни присвоявания на роли
- Уверете се, че потребителят е влязъл след промените в ролите