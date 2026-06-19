---
## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'deleteUserBadge Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const badgeId: string = 'badge_9f8b2c1d';
const includeAudit: boolean | undefined = undefined; // valgfri flag (ikke påkrævet af deleteUserBadge)
const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
[inline-code-end]

---