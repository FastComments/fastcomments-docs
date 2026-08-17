[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Цей API‑ендпоінт надає можливість створювати шаблони електронної пошти.

Примітки:

- Ви не можете мати кілька шаблонів з однаковим `emailTemplateId` у тому ж домені.
- Але ви можете мати шаблон‑заміни (`domain` = `*`) та домен‑специфічний шаблон для того ж `emailTemplateId`.
- Вказування `domain` має сенс лише якщо у вас є різні домени або ви хочете використовувати конкретні шаблони для тестування (`domain` встановлено на `localhost` тощо).
- Якщо ви вказуєте `domain`, він має відповідати `DomainConfig`. При помилці надається список дійсних доменів.
- Синтаксис шаблону — EJS, і він рендериться з тайм‑аутом 500 мс. P99 для рендерингу <5 мс, тому якщо ви досягаєте 500 мс, щось не так.
- **Ваш шаблон повинен рендеритися з вашими `testData`**, щоб зберегти. Помилки рендерингу агрегуються та відображаються у панелі (скоро буде доступно через API). 

Мінімальні дані, необхідні для додавання шаблону, виглядають так:

[inline-code-attrs-start title = 'Мінімальний приклад POST cURL для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I\'m a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Ви можете захотіти мати шаблони для кожного сайту, у цьому випадку ви визначаєте `domain`:

[inline-code-attrs-start title = 'Приклад POST cURL для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I\'m a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту POST для EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді POST для EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---