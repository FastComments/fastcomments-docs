## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| deleteComments | string | Не |  |
| commentDeleteMode | string | Не |  |

## Отговор

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-789", deleteComments = "true", commentDeleteMode = "soft")
if response.isSome:
  let apiResp = response.get()
  echo "Tenant user deleted, response: ", apiResp
else:
  echo "Failed to delete tenant user, HTTP status: ", $httpResponse.status
[inline-code-end]

---