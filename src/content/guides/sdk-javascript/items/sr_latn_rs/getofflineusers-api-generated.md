Prošli komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Kursorska paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName}
indeks od afterName naprijed pomoću $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]