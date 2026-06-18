## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nein |  |
| limit | number | Nein |  |
| skip | number | Nein |  |
| asTree | boolean | Nein |  |
| skipChildren | number | Nein |  |
| limitChildren | number | Nein |  |
| maxTreeDepth | number | Nein |  |
| urlId | string | Nein |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |
| contextUserId | string | Nein |  |
| hashTag | string | Nein |  |
| parentId | string | Nein |  |
| direction | SortDirections | Nein |  |
| fromDate | number | Nein |  |
| toDate | number | Nein |  |

## Antwort

Gibt zurück: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---