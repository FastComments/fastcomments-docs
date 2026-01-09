## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postIds | seq[string] | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[GetUserReactsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_reacts_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getUserReactsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(tenantId = "my-tenant-123", postIds = @[], sso = "")
if response.isSome:
  let reacts = response.get()
  discard reacts
[inline-code-end]

---