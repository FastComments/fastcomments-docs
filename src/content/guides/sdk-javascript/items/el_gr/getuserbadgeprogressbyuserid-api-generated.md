## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα για getUserBadgeProgressByUserId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9c2d3b';
const maybeUserId: string | undefined = 'user_4b8e1f9a'; // optional source (could be undefined)
const userId: string = maybeUserId ?? 'user_fallback0001';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);
console.log(result);
[inline-code-end]

---