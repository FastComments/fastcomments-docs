Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Използва се от уиджета за коментари за обогатяване на потребителите, които току-що са се появили чрез събитие за присъствие.  
No page context: privacy is enforced uniformly (private profiles are masked).  
Без контекст на страницата: поверителността се налага еднородно (частните профили се маскират).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Не |  |

## Отговор

Връща: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Пример

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]