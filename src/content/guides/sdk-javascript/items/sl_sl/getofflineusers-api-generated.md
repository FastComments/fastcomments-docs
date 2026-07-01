Prejšnji komentatorji na strani, ki trenutno NI online. Razvrščeni po **displayName**.  
Uporabite to po tem, ko ste izčrpali `/users/online`, za prikaz sekcije “Člani”.  
Kazalna paginacija po **commenterName**: strežnik prehaja delni `{tenantId, urlId, commenterName}` indeks od **afterName** naprej prek `$gt`, brez stroška `$skip`.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vrne: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Primer

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---