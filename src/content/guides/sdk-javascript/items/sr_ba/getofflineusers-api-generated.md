---
Prethodni komentatori na stranici koji NISU trenutno online. Sortirani po displayName.  
Koristite ovo nakon što iscrpite /users/online za prikazivanje sekcije "Members".  
Paginacija kursora po commenterName: server prolazi kroz djelimični {tenantId, urlId, commenterName} indeks od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Odgovor

Vraća: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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