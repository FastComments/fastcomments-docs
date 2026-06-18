## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |
| badgeId | string | Όχι |  |
| type | number | Όχι |  |
| displayedOnComments | boolean | Όχι |  |
| limit | number | Όχι |  |
| skip | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9a12';
const userId: string = 'user_42b7';
const badgeId: string = 'badge_top_contributor';
const type: number = 2;
const displayedOnComments: boolean = true;
const limit: number = 25;
const skip: number = 0;
const badges: GetUserBadges200Response = await getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip);
[inline-code-end]

---