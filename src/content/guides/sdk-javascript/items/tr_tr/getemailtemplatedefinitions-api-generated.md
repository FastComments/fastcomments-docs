## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |

## Yanıt

Döndürür: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// İsteğe bağlı parametreler (destekleniyorsa) ikinci argüman olarak geçirilebilir, örn. getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]