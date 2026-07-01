## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| options | GetUserReactsPublicOptions | Не |  |

## Отговор

Връща: [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getUserReactsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetUserReactsPublicOptions(
  limit = 20,
  offset = 0,
  includeDeleted = false
)

let (response, httpResponse) = client.getUserReactsPublic(
  tenantId = "my-tenant-123",
  options = opts
)

if response.isSome:
  let userReacts = response.get()
[inline-code-end]