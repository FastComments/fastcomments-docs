## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| page | number | Hayır |  |
| limit | number | Hayır |  |
| skip | number | Hayır |  |
| asTree | boolean | Hayır |  |
| skipChildren | number | Hayır |  |
| limitChildren | number | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| urlId | string | Hayır |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |
| contextUserId | string | Hayır |  |
| hashTag | string | Hayır |  |
| parentId | string | Hayır |  |
| direction | SortDirections | Hayır |  |
| fromDate | number | Hayır |  |
| toDate | number | Hayır |  |

## Yanıt

Döndürür: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getComments Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---