## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vrne: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUserBadge Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBadge() {
    const tenantId: string = "tenant-9f8b7c6d";
    const userId: string = "user-123456";
    const badgeResponse: APIGetUserBadgeResponse = await getUserBadge(tenantId, userId);
}
[inline-code-end]

---