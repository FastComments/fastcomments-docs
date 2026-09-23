## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| userId | string | Nein |  |
| state | number | Nein |  |
| skip | number | Nein |  |
| limit | number | Nein |  |

## Antwort

Rückgabe: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getTickets Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const ticketsSimple: GetTicketsResponse = await getTickets(tenantId);

  const userId: string = "user_9876";
  const state: number = 1; // z.B., offen
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