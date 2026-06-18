## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |
| badgeId | string | Nie |  |
| type | number | Nie |  |
| displayedOnComments | boolean | Nie |  |
| limit | number | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## Przykład

[inline-code-attrs-start title = 'getUserBadges Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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