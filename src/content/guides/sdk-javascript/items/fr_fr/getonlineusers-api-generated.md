Visiteurs actuellement en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés de la salle, y compris les visiteurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| afterName | string | Non |  |
| afterUserId | string | Non |  |

## Réponse

Renvoie: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---