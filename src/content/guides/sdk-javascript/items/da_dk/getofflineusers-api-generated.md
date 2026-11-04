Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug denne efter at have brugt /users/online op til at rendere et "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren gennemgår det partielle {tenantId, urlId, commenterName}
indeks fra afterName fremad via $gt, uden $skip-omkostninger.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nej |  |
| afterUserId | string | Nej |  |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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