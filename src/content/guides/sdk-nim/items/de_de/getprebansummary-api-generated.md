## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| includeByUserIdAndEmail | bool | Nein |  |
| includeByIP | bool | Nein |  |
| includeByEmailDomain | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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