## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nej |  |
| limit | number | Nej |  |
| skip | number | Nej |  |
| asTree | boolean | Nej |  |
| skipChildren | number | Nej |  |
| limitChildren | number | Nej |  |
| maxTreeDepth | number | Nej |  |
| urlId | string | Nej |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |
| contextUserId | string | Nej |  |
| hashTag | string | Nej |  |
| parentId | string | Nej |  |
| direction | SortDirections | Nej |  |
| fromDate | number | Nej |  |
| toDate | number | Nej |  |

## Svar

Returnerer: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---