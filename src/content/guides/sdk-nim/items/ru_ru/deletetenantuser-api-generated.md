## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| deleteComments | string | Нет |  |
| commentDeleteMode | string | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

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