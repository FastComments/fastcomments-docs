## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Не |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |

## Одговор

Враћа: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'unBlockUserFromComment Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "news-site-001",
  id = "cmt-8fj3k9",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-98765",
  anonUserId = ""
)

if response.isSome:
  let unblocked = response.get()
  discard unblocked
[inline-code-end]

---