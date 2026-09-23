## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Antwort

Rückgabe: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateUserBadge Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const id: string = "user-4a3b2c1d";

const updateParams: UpdateUserBadgeParams = {
  badgeId: "badge-premium",
  // optionales Feld
  expiresAt: new Date("2025-12-31T23:59:59Z")
};

const result: APIEmptySuccessResponse = await updateUserBadge(tenantId, id, updateParams);
[inline-code-end]