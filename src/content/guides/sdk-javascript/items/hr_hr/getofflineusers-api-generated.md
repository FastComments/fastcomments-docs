Past komentatori na stranici koji NISU trenutno online. Sortirano po displayName.  
Koristite ovo nakon što iscrpite /users/online za prikaz odjeljka „Članovi”.  
Kursor paginacija po commenterName: poslužitelj prolazi kroz djelomični {tenantId, urlId, commenterName} indeks od afterName naprijed putem $gt, bez troška $skip.

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---