## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| state | number | Ne |  |
| skip | number | Ne |  |
| limit | number | Ne |  |

## Odgovor

Vraća: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## Primer

[inline-code-attrs-start title = 'getTickets Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const ticketsSimple: GetTicketsResponse = await getTickets(tenantId);

  const userId: string = "user_9876";
  const state: number = 1; // npr., otvoren
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