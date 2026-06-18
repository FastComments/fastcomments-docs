Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en "Medlemmer" sektion.
Cursor-paginering på commenterName: serveren går igennem det delvise {tenantId, urlId, commenterName} index fra afterName fremad via $gt, uden $skip-omkostning.

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nej |  |
| afterUserId | string | Nej |  |

## Svar

Returnerer: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---