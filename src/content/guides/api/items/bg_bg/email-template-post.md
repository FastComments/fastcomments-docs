[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за създаване на имейл шаблони.

Забележки:

- Не можете да имате множество шаблони с един и същ `emailTemplateId` с един и същ домейн.
- Но можете да имате универсален шаблон (`domain` = `*`) и специфичен за домейн шаблон за същия `emailTemplateId`.
- Посочването на `domain` е уместно само ако имате различни домейни или искате да използвате специфични шаблони за тестване (`domain` зададен на `localhost` и т.н.).
- Ако посочите `domain`, той трябва да съвпада с `DomainConfig`. При грешка се предоставя списък с валидни домейни.
- Синтаксисът на шаблона е EJS и се визуализира с таймаут от 500ms. P99 за визуализация е <5ms, така че ако достигнете 500ms, нещо не е наред.
- **Вашият шаблон трябва да се визуализира с вашите дадени `testData`**, за да се запази. Грешките при визуализация се агрегират и докладват в таблото (скоро налично чрез API).

Минималните данни, необходими за добавяне на шаблон, са следните:

[inline-code-attrs-start title = 'Пример за минимален POST на EmailTemplate с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Може да искате да имате шаблони за всеки сайт, в който случай дефинирате `domain`:

[inline-code-attrs-start title = 'Пример за POST на EmailTemplate с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за POST на EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за POST на EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
