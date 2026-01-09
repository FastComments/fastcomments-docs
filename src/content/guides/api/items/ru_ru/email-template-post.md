[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность создавать шаблоны электронных писем.

Примечания:

- У вас не может быть нескольких шаблонов с одинаковым `emailTemplateId` в одном и том же домене.
- Но вы можете иметь шаблон-джокер (`domain` = `*`) и шаблон, специфичный для домена, для того же `emailTemplateId`.
- Указание `domain` имеет значение только если у вас несколько доменов или вы хотите использовать специфичные шаблоны для тестирования (например, `domain` = `localhost`).
- Если вы указываете `domain`, он должен соответствовать `DomainConfig`. В случае ошибки предоставляется список допустимых доменов.
- Синтаксис шаблонов — EJS, и рендеринг выполняется с таймаутом 500 мс. P99 времени рендеринга <5ms, поэтому если вы достигаете 500ms — что-то не так.
- **Ваш шаблон должен успешно рендериться с предоставленными `testData`** для сохранения. Ошибки рендеринга агрегируются и отображаются на панели (вскоре будет доступно через API). 

Минимальные данные, необходимые для добавления шаблона, следующие:

[inline-code-attrs-start title = 'Минимальный пример cURL POST для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Возможно, вы захотите иметь шаблоны для каждого сайта, в этом случае вы указываете `domain`:

[inline-code-attrs-start title = 'Пример cURL POST для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура POST-запроса EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа POST для EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Включается при ошибке. **/
    reason?: string
    /** Созданный шаблон. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---