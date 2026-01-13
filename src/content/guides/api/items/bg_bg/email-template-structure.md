Обектът `EmailTemplate` представлява конфигурация за персонализиран имейл шаблон за tenant.

Системата ще избере имейл шаблона за използване чрез:

- Неговия идентификатор на типа, наречен `emailTemplateId`. Това са константи.
- `domain`. Първо ще се опитаме да намерим шаблон за домейна, към който е свързан съответният обект (като `Comment`), и ако не бъде намерено съвпадение, ще се опитаме да намерим шаблон, където domain е null или `*`.

Структурата на обекта `EmailTemplate` е следната:

[inline-code-attrs-start title = 'Структура на Email Template'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** The domain the template should be associated with. **/
    domain?: string | '*' | null
    /** The email template content in EJS syntax. **/
    ejs: string
    /** A map of overridden translation keys to values, for each supported locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** An object that represents the render context of the template. **/
    testData: object
}
[inline-code-end]

### Забележки

- Можете да получите валидните стойности на `emailTemplateId` от крайната точка `/definitions`.
- Крайната точка `/definitions` също включва преводите по подразбиране и тестовите данни.
- Шаблоните няма да се запазят с невалидна структура или тестови данни.
