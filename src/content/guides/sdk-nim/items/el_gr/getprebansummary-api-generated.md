## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Ναι |  |
| includeByUserIdAndEmail | bool | Όχι |  |
| includeByIP | bool | Όχι |  |
| includeByEmailDomain | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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