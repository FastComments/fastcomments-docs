## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| options | GetPreBanSummaryOptions | Ні |  |

## Ответ

Возвращает: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Приклад

[inline-code-attrs-start title = 'getPreBanSummary Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (preBanSummaryOpt, httpResponse) = client.getPreBanSummary(tenantId = "my-tenant-123", commentId = "cmt-456", options = GetPreBanSummaryOptions())
if preBanSummaryOpt.isSome:
  let summary = preBanSummaryOpt.get()
[inline-code-end]