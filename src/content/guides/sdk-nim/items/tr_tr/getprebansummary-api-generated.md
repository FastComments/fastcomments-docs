## Parametreler

| Name | Type | Required | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| includeByUserIdAndEmail | bool | Hayır |  |
| includeByIP | bool | Hayır |  |
| includeByEmailDomain | bool | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Dönen: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Örnek

[inline-code-attrs-start title = 'getPreBanSummary Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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