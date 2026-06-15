---
Prejšnji komentatorji na strani, ki TRENUTNO NISO na spletu. Razvrščeno po displayName.
Uporabite to po izčrpanju /users/online za prikaz razdelka "Člani".
Kursorna paginacija na commenterName: strežnik prehaja po delnem {tenantId, urlId, commenterName} indeksu od afterName naprej z $gt, brez stroška $skip.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vrača: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---