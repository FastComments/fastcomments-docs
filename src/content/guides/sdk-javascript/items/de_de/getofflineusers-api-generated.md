Frühere Kommentatoren auf der Seite, die aktuell NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies nach dem Ausschöpfen von /users/online, um einen „Mitglieder"-Bereich darzustellen.
Cursor-Paginierung über commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts über $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Beispiel

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