[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава креирање шаблона е-поште.

Напомене:

- Не можете имати више шаблона са истим `emailTemplateId` за исти домен.
- Међутим, можете имати џокер-шаблон (`domain` = `*`) и шаблон специфичан за домен за исти `emailTemplateId`.
- Навођење `domain` је релевантно само ако имате различите домене, или желите користити специфичне шаблоне за тестирање (`domain` постављен на `localhost` итд).
- Ако наведете `domain`, он мора одговарати `DomainConfig`. У случају грешке биће обезбеђен списак важећих домена.
- Синтакса шаблона је EJS и рендерује се уз тајмаут од 500ms. P99 за рендеровање је <5ms, тако да ако достигнете 500ms нешто није у реду.
- **Ваш шаблон мора да се рендерује са датим `testData`** да би се сачувао. Грешке приликом рендеровања се агрегирају и пријављују у контролној табли (ускоро доступно и преко API-а). 

Минимални подаци потребни за додавање шаблона су следећи:

[inline-code-attrs-start title = 'Минимални cURL пример за EmailTemplate POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

You may want to have templates per-site, in which case you define `domain`:

[inline-code-attrs-start title = 'cURL пример за EmailTemplate POST'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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