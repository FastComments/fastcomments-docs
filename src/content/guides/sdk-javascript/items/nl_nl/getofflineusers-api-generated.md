Past commenters on the page who are NOT currently online. Sorted by displayName.  
Eerdere commentatoren op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.  

Use this after exhausting /users/online to render a "Members" section.  
Gebruik dit nadat /users/online is uitgeput om een "Members"-sectie weer te geven.  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Cursor-paginering op commenterName: server doorloopt de partiële {tenantId, urlId, commenterName} index vanaf afterName vooruit via $gt, zonder $skip‑kost.  

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Respons

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