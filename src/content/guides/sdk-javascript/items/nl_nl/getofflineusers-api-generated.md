---
Vorige reageerders op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit nadat je /users/online hebt uitgeput om een 'Leden'-sectie weer te geven.
Cursor-paginatie op commenterName: de server doorloopt het gedeeltelijke {tenantId, urlId, commenterName}
index vanaf afterName vooruit via $gt, geen $skip-kosten.

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---