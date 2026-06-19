## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Retourneert: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'deleteUserBadge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const badgeId: string = 'badge_9f8b2c1d';
const includeAudit: boolean | undefined = undefined; // optionele vlag (niet vereist voor deleteUserBadge)
const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
[inline-code-end]

---