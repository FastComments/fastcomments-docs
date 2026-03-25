## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nein |  |
| state | number | Nein |  |
| skip | number | Nein |  |
| limit | number | Nein |  |

## Antwort

Gibt zurück: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getTickets Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92f3b4c1";
const userId: string = "user_742a9f3e";
const state: number = 1;
const skip: number = 0;
const limit: number = 25;
const ticketsFull: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
const ticketsMinimal: GetTickets200Response = await getTickets("tenant_92f3b4c1");
[inline-code-end]