## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| editKey | string | Не |  |

## Отговор

Връща: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "", editKey = "")
if response.isSome:
  let deleted = response.get()
  discard deleted
[inline-code-end]

---