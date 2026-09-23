## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| userId | string | いいえ |  |

## レスポンス

返却: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse.ts)

## 例

[inline-code-attrs-start title = 'getTicket の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTickets() {
  const tenantId: string = "acme-corp";
  const ticketId: string = "ticket-12345";
  const userId: string = "user-9876";

  const ticketWithUser: GetTicketResponse = await getTicket(tenantId, ticketId, userId);
  const ticketWithoutUser: GetTicketResponse = await getTicket(tenantId, ticketId);
}
[inline-code-end]

---