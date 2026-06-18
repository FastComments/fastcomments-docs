Vorige commentatoren op de pagina die OP DIT MOMENT NIET online zijn. Gesorteerd op displayName.
Gebruik dit nadat u /users/online hebt uitgeput om een "Leden"-sectie weer te geven.
Cursor-paginering op commenterName: de server doorloopt de gedeeltelijke {tenantId, urlId, commenterName} index vanaf afterName vooruit via $gt, geen $skip-kosten.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Response

Retourneert: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---