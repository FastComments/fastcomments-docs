## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| sure | string | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'deleteTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42c9f1';
const id: string = 'flag_9a7b3c';
const sure: string = 'confirm-delete';
const result: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
[inline-code-end]

---