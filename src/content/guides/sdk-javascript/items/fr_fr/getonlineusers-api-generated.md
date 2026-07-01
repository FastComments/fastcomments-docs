---
Visionneurs actuellement en ligne d'une page : personnes dont la session WebSocket est abonnée à la page en ce moment.  
Renvoie anonCount + totalCount (abonnés à l'ensemble de la salle, y compris les visionneurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Réponse

Renvoie : [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // Avec des paramètres de pagination optionnels
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Sans paramètres de pagination optionnels
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]

---