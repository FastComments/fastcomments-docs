[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası e-posta şablonları oluşturma olanağı sağlar.

Notlar:

- Aynı domain ile aynı `emailTemplateId`'ye sahip birden fazla şablon olamaz.
- Ancak bir joker şablon (`domain` = `*`) ile aynı `emailTemplateId` için alan adı özel bir şablon birlikte bulunabilir.
- `domain` belirtmek yalnızca farklı alan adlarınız varsa veya test için belirli şablonlar kullanmak istiyorsanız önemlidir (`domain` set to `localhost` etc).
- `domain` belirtirseniz, bunun bir `DomainConfig` ile eşleşmesi gerekir. Hata durumunda geçerli alan adlarının bir listesi sağlanır.
- Şablon sözdizimi EJS'tir ve 500ms zaman aşımı ile render edilir. Render süresi için P99 <5ms'dir, bu nedenle 500ms'ye ulaşıyorsanız bir sorun var demektir.
- **Kaydetmek için şablonunuz verilen `testData` ile render olmalıdır**. Render hataları birleştirilir ve kontrol panelinde raporlanır (yakında API üzerinden de kullanılabilir).

Bir şablon eklemek için gereken minimum veriler aşağıdaki gibidir:

[inline-code-attrs-start title = 'Minimum EmailTemplate POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Site başına şablonlar isteyebilirsiniz; bu durumda `domain` tanımlarsınız:

[inline-code-attrs-start title = 'EmailTemplate POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'EmailTemplate POST İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Hata durumunda dahil edilir. **/
    reason?: string
    /** Oluşturulan şablon. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]