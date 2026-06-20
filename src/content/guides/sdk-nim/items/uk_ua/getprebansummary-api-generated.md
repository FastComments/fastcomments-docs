## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| commentId | string | Так |  |
| includeByUserIdAndEmail | bool | Ні |  |
| includeByIP | bool | Ні |  |
| includeByEmailDomain | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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