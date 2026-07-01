## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| options | PostBanUserFromCommentOptions | Нет |  |

## Ответ

Возвращает: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (banResult, httpResp) = client.postBanUserFromComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  options = PostBanUserFromCommentOptions()
)

if banResult.isSome:
  let result = banResult.get()
  echo result
[inline-code-end]