## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | GetPreBanSummaryOptions | No |  |

## Одговор

Враћа: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Пример

[inline-code-attrs-start title = 'Primer getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (preBanSummaryOpt, httpResponse) = client.getPreBanSummary(tenantId = "my-tenant-123", commentId = "cmt-456", options = GetPreBanSummaryOptions())
if preBanSummaryOpt.isSome:
  let summary = preBanSummaryOpt.get()
[inline-code-end]