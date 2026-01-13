Объект `EmailTemplate` представляет конфигурацию для пользовательского шаблона электронной почты для тенанта.

Система будет выбирать шаблон электронной почты по:

- Его идентификатору типа, мы называем это `emailTemplateId`. Они являются константами.
- `domain`. Сначала мы попытаемся найти шаблон для домена, к которому привязан связанный объект (например, `Comment`), и если совпадение не найдено, то попытаемся найти шаблон, где domain равен null или `*`.

Структура для объекта `EmailTemplate` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура шаблона электронной почты'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Отображение переопределённых ключей переводов в значения для каждого поддерживаемого языка. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Объект, представляющий контекст рендеринга шаблона. **/
    testData: object
}
[inline-code-end]

### Примечания

- Вы можете получить допустимые значения `emailTemplateId` через endpoint `/definitions`.
- Endpoint `/definitions` также содержит переводы по умолчанию и тестовые данные.
- Сохранение шаблона не удастся при неверной структуре или некорректных тестовых данных.