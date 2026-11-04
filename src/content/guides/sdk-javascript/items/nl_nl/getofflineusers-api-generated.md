Eerdere commenters op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit na het uitputten van /users/online om een "Leden"-sectie weer te geven.
Cursorpaginering op commenterName: de server loopt via $gt door de partiële {tenantId, urlId, commenterName}
index vanaf afterName vooruit, zonder $skip-kosten.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Response

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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