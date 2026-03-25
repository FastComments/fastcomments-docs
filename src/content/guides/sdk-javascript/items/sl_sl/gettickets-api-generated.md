## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| state | number | Ne |  |
| skip | number | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vrača: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getTickets'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92f3b4c1";
const userId: string = "user_742a9f3e";
const state: number = 1;
const skip: number = 0;
const limit: number = 25;
const ticketsFull: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
const ticketsMinimal: GetTickets200Response = await getTickets("tenant_92f3b4c1");
[inline-code-end]

---