## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| options | GetPreBanSummaryOptions | 아니오 |  |

## 응답

반환: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## 예제

[inline-code-attrs-start title = 'getPreBanSummary 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (preBanSummaryOpt, httpResponse) = client.getPreBanSummary(tenantId = "my-tenant-123", commentId = "cmt-456", options = GetPreBanSummaryOptions())
if preBanSummaryOpt.isSome:
  let summary = preBanSummaryOpt.get()
[inline-code-end]