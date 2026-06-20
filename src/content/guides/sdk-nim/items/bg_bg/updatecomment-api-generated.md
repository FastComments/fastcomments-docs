## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updatableCommentParams | UpdatableCommentParams | Не |  |
| contextUserId | string | Не |  |
| doSpamCheck | bool | Не |  |
| isLive | bool | Не |  |

## Отговор

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за updateComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654",
  updatableCommentParams = UpdatableCommentParams(
    text = "Updated comment: corrected facts and clarified wording.",
    isApproved = true,
    tags = @["news", "update"]
  ),
  contextUserId = "user-456",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let apiResp = response.get()
  discard apiResp
[inline-code-end]

---