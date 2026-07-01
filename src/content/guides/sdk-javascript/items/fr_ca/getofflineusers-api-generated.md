Commentateurs précédents sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.  
Utilisez ceci après avoir épuisé /users/online pour rendre une section « Membres ».  
Pagination par curseur sur commenterName : le serveur parcourt le partiel {tenantId, urlId, commenterName}  
indice à partir de afterName vers l’avant via $gt, aucun coût $skip.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Réponse

Renvoie : [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]