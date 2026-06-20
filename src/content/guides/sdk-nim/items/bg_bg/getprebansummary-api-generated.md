## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Да |  |
| includeByUserIdAndEmail | bool | Не |  |
| includeByIP | bool | Не |  |
| includeByEmailDomain | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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