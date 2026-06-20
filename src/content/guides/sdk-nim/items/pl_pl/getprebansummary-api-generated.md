## Parametry

| Name | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| commentId | string | Tak |  |
| includeByUserIdAndEmail | bool | Nie |  |
| includeByIP | bool | Nie |  |
| includeByEmailDomain | bool | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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