---
Visiteurs actuellement en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés de la salle, y compris les spectateurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| afterName | string | Non |  |
| afterUserId | string | Non |  |

## Réponse

Renvoie : [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---