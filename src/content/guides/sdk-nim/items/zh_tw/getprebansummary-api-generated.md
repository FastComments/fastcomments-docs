## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| includeByUserIdAndEmail | bool | 否 |  |
| includeByIP | bool | 否 |  |
| includeByEmailDomain | bool | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## 範例

[inline-code-attrs-start title = 'getPreBanSummary 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentId = "cmt-7423"
let (response, httpResponse) = client.getPreBanSummary(
  commentId = commentId,
  includeByUserIdAndEmail = false,
  includeByIP = false,
  includeByEmailDomain = false,
  sso = ""
)
if response.isSome:
  let preBanSummary = response.get()
  discard preBanSummary
else:
  discard httpResponse
[inline-code-end]