## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | はい |  |
| id | string | はい |  |
| changeTicketStateBody | ChangeTicketStateBody | はい |  |

## 応答

返却: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## 例

[inline-code-attrs-start title = 'changeTicketState の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const ticketId: string = "ticket-20230915-001";

  const changeTicketStateBody: ChangeTicketStateBody = {
    // オプションフィールドの例
    note: "Resolved after investigation"
  };

  const response: ChangeTicketStateResponse = await changeTicketState(
    tenantId,
    userId,
    ticketId,
    changeTicketStateBody
  );

  console.log(response);
})();
[inline-code-end]

---