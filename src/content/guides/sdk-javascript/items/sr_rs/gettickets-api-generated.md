## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| state | number | No |  |
| skip | number | No |  |
| limit | number | No |  |

## Одговор

Vraća: [`GetTicketsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getTickets'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadTickets() {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const state: number = 2; // npr., zatvoreno
  const skip: number = 10;
  const limit: number = 5;

  const ticketsFull: GetTicketsResponse1 = await getTickets(tenantId, userId, state, skip, limit);
  const ticketsPartial: GetTicketsResponse1 = await getTickets(tenantId);
}

loadTickets();
[inline-code-end]