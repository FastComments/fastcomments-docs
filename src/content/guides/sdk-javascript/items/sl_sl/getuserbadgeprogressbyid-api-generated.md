---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrne: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUserBadgeProgressById Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const badgeId: string = "badge-2024-07";

const result: APIGetUserBadgeProgressResponse = await getUserBadgeProgressById(tenantId, badgeId);

const progress: UserBadgeProgress | undefined = result?.progress;
const status: APIStatus | undefined = result?.status;
[inline-code-end]

---