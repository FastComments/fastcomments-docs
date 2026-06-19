Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at gengive en "Medlemmer"-sektion.
Kursorpaginering på commenterName: serveren går igennem den delvise {tenantId, urlId, commenterName}
indeks fra afterName fremad via $gt, uden $skip-omkostning.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nej |  |
| afterUserId | string | Nej |  |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---