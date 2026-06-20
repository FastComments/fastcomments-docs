## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| includeByUserIdAndEmail | bool | 아니요 |  |
| includeByIP | bool | 아니요 |  |
| includeByEmailDomain | bool | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## 예제

[inline-code-attrs-start title = 'getPreBanSummary 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---