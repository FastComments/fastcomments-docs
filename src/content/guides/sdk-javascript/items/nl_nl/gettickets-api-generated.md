## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| state | number | Nee |  |
| skip | number | Nee |  |
| limit | number | Nee |  |

## Respons

Retourneert: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTickets Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const ticketsSimple: GetTicketsResponse = await getTickets(tenantId);

  const userId: string = "user_9876";
  const state: number = 1; // bijv., open
  const skip: number = 0;
  const limit: number = 20;
  const ticketsFull: GetTicketsResponse = await getTickets(
    tenantId,
    userId,
    state,
    skip,
    limit
  );
})();
[inline-code-end]

---