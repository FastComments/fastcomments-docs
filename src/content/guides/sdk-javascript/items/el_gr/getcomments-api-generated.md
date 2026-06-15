## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | number | Όχι |  |
| limit | number | Όχι |  |
| skip | number | Όχι |  |
| asTree | boolean | Όχι |  |
| skipChildren | number | Όχι |  |
| limitChildren | number | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| urlId | string | Όχι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |
| contextUserId | string | Όχι |  |
| hashTag | string | Όχι |  |
| parentId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| fromDate | number | Όχι |  |
| toDate | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---