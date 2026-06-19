## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| state | number | Ne |  |
| skip | number | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vraća: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getTickets-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises";
const userId: string | undefined = "u_56321";
const state: number | undefined = 1;
const skip: number = 0;
const limit: number = 50;
const response: GetTicketsResponse = await getTickets(tenantId, userId, state, skip, limit);
[inline-code-end]

---