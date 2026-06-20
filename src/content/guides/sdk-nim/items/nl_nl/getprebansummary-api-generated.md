## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| commentId | string | Ja |  |
| includeByUserIdAndEmail | bool | Nee |  |
| includeByIP | bool | Nee |  |
| includeByEmailDomain | bool | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getPreBanSummary Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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