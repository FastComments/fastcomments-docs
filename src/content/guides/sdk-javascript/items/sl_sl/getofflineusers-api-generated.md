Prejšnji komentatorji na strani, ki trenutno niso online. Razvrščeni po displayName.
Uporabite to po tem, ko izčrpate /users/online, da prikažete razdelek "Člani".
Kazalno (cursor) straničenje po commenterName: strežnik hodi po delnem {tenantId, urlId, commenterName}
indeksu od afterName naprej z uporabo $gt, brez stroška $skip.

## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vrača: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer uporabe getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]