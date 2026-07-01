Prethodni komentatori na stranici koji NISTE trenutno online. Sortirani po displayName.  
Koristite ovo nakon što ste iscrpili /users/online da prikažete sekciju „Members“.  
Kursorska paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName}  
indeks od afterName napred putem $gt, bez troška $skip.

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Response

Vraća: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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