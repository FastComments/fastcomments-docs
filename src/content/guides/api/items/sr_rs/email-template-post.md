[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Ова API крајња тачка омогућава креирање шаблона е-поште.

Напомене:

- Не можете имати више шаблона са истим `emailTemplateId` за исти домен.
- Међутим, можете имати wildcard шаблон (`domain` = `*`) и домен-специфичан шаблон за исти `emailTemplateId`.
- Навођење `domain` је релевантно само ако имате више домена, или желите да користите специфичне шаблоне за тестирање (`domain` подешен на `localhost` итд).
- Ако наведете `domain`, он мора да одговара `DomainConfig`. У случају грешке биће дат списак валидних домена.
- Синтакса шаблона је EJS и рендерује се са timeout-ом од 500ms. P99 за рендеровање је <5ms, па ако достигнете 500ms нешто је погрешно.
- **Ваш шаблон мора да се рендерује са датим `testData`** да би се сачувао. Грешке приликом рендеровања се агрегирају и пријављују на контролној табли (ускоро доступно преко API-ја). 

Минимални подаци потребни за додавање шаблона су следећи:

[inline-code-attrs-start title = 'Минималан EmailTemplate POST cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Можда ћете желети шаблоне по сајту, у том случају дефинишете `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура EmailTemplate POST захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура EmailTemplate POST одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Креирани шаблон. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---