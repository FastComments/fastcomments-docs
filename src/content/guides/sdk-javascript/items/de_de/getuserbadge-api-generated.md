## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserBadge Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-72a1';
const id: string = 'badge_5d8f3c9';
const response: APIGetUserBadgeResponse = await getUserBadge(tenantId, id);
const status: APIStatus = response.status;
const badgeTitle: string | undefined = response.userBadge?.title;
[inline-code-end]

---