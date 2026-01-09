[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность создавать шаблоны электронной почты.

Примечания:

- Нельзя иметь несколько шаблонов с одинаковым `emailTemplateId` для одного и того же домена.
- Однако можно иметь шаблон с подстановочным символом (`domain` = `*`) и шаблон, специфичный для домена, для того же `emailTemplateId`.
- Указание `domain` имеет смысл только если у вас несколько доменов или вы хотите использовать отдельные шаблоны для тестирования (`domain`, например, `localhost`).
- Если вы указываете `domain`, он должен соответствовать `DomainConfig`. В случае ошибки будет предоставлен список допустимых доменов.
- Синтаксис шаблонов — EJS, и они рендерятся с тайм-аутом 500 мс. P99 времени рендеринга <5 мс, поэтому если вы достигаете 500 мс — что-то не так.
- **Ваш шаблон должен успешно рендериться с заданным `testData`**, чтобы его можно было сохранить. Ошибки рендеринга агрегируются и отображаются на панели управления (скоро будет доступно через API). 

Минимальные данные, необходимые для добавления шаблона, приведены ниже:

[inline-code-attrs-start title = 'Минимальный пример POST cURL для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Возможно, вы захотите иметь шаблоны для каждого сайта, в этом случае вы задаёте `domain`:

[inline-code-attrs-start title = 'Пример POST cURL для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура POST-ответа EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Указывается при ошибке. **/
    reason?: string
    /** Созданный шаблон. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---