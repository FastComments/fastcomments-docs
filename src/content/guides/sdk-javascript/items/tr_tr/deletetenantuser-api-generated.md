## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteTenantUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7a9f2c";
const id: string = "user_9d3f1b6a";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: APIEmptyResponse = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]

---