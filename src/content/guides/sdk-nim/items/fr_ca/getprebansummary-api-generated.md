## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| includeByUserIdAndEmail | bool | Non |  |
| includeByIP | bool | Non |  |
| includeByEmailDomain | bool | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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