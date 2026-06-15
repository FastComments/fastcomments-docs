Prošli komentatori na stranici koji trenutno nisu online. Sortirano po displayName.
Koristite ovo nakon iscrpljivanja /users/online za prikaz odjeljka "Članovi".
Kursor paginacija po commenterName: server prolazi djelomični indeks {tenantId, urlId, commenterName}
index od afterName prema naprijed koristeći $gt, bez troška $skip.

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vraća: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---