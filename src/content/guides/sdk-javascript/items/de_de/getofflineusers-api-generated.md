Past commenters on the page who are NOT currently online. Sorted by displayName.  
Frühere Kommentatoren auf der Seite, die NICHT gerade online sind. Sortiert nach displayName.  

Use this after exhausting /users/online to render a "Members" section.  
Verwenden Sie dies, nachdem /users/online erschöpft wurde, um einen „Members“-Abschnitt darzustellen.  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Cursor-Paginierung nach commenterName: Der Server durchläuft den Teil {tenantId, urlId, commenterName} Index von afterName ausgehend vorwärts mittels $gt, ohne $skip‑Kosten.  

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Response

Rückgabe: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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