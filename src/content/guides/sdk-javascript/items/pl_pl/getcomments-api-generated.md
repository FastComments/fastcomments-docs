## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| page | number | Nie |  |
| limit | number | Nie |  |
| skip | number | Nie |  |
| asTree | boolean | Nie |  |
| skipChildren | number | Nie |  |
| limitChildren | number | Nie |  |
| maxTreeDepth | number | Nie |  |
| urlId | string | Nie |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |
| contextUserId | string | Nie |  |
| hashTag | string | Nie |  |
| parentId | string | Nie |  |
| direction | SortDirections | Nie |  |
| fromDate | number | Nie |  |
| toDate | number | Nie |  |

## Odpowiedź

Zwraca: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---