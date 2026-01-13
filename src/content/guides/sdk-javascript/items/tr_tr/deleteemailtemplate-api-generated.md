---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'deleteEmailTemplate Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b2f6a-4b2f6a2d";
const templateId: string = "email_template_9f8b7c3e";
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, templateId);
const status: APIStatus | undefined = result?.status
[inline-code-end]

---