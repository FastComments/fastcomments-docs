## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| blockFromCommentParams | BlockFromCommentParams | Не |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |

## Отговор

Връща: [`Option[BlockFromCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_from_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за blockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-98765",
  blockFromCommentParams = BlockFromCommentParams(),
  userId = "user-456",
  anonUserId = ""
)
if response.isSome:
  let blocked = response.get()
  echo "Block confirmed for tenant:", " my-tenant-123"
[inline-code-end]

---