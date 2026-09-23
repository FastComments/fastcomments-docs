## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| id | string | 是 |  |
| changeTicketStateBody | ChangeTicketStateBody | 是 |  |

## 回應

返回：[`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## 範例

[inline-code-attrs-start title = 'changeTicketState 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const ticketId: string = "ticket-20230915-001";

  const changeTicketStateBody: ChangeTicketStateBody = {
    // 可選欄位範例
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