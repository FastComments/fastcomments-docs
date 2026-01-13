---
Объект `EmailTemplate` представляет конфигурацию пользовательского шаблона письма для тенанта.

Система выберет шаблон письма следующим образом:

- По его идентификатору типа, который мы называем `emailTemplateId`. Это константы.
- Домен (`domain`). Сначала мы попытаемся найти шаблон для домена, к которому привязан связанный объект (например, `Comment`), и если совпадение не найдено, то попытаемся найти шаблон, где domain равен null или `*`.

Структура объекта `EmailTemplate` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура шаблона письма'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ **/
    createdAt: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ **/
    updatedAt: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ **/
    updatedByUserId: string
    /** Домен, с которым должен быть связан шаблон. **/
    domain?: string | '*' | null
    /** Содержимое шаблона письма в синтаксисе EJS. **/
    ejs: string
    /** Отображение переопределённых ключей переводов в значения для каждой поддерживаемой локали. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Объект, представляющий контекст рендеринга шаблона. **/
    testData: object
}
[inline-code-end]

### Примечания

- Вы можете получить допустимые значения `emailTemplateId` через эндпоинт `/definitions`.
- Эндпоинт `/definitions` также включает переводы по умолчанию и тестовые данные.
- Шаблоны не сохранятся при недопустимой структуре или тестовых данных.

---