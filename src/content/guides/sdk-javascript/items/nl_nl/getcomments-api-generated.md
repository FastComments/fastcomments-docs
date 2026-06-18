## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nee |  |
| limit | number | Nee |  |
| skip | number | Nee |  |
| asTree | boolean | Nee |  |
| skipChildren | number | Nee |  |
| limitChildren | number | Nee |  |
| maxTreeDepth | number | Nee |  |
| urlId | string | Nee |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |
| contextUserId | string | Nee |  |
| hashTag | string | Nee |  |
| parentId | string | Nee |  |
| direction | SortDirections | Nee |  |
| fromDate | number | Nee |  |
| toDate | number | Nee |  |

## Respons

Geeft terug: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---