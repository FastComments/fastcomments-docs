---
## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| deleteComments | string | Ні |  |
| commentDeleteMode | string | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-789", deleteComments = "true", commentDeleteMode = "soft")
if response.isSome:
  let apiResp = response.get()
  echo "Tenant user deleted, response: ", apiResp
else:
  echo "Failed to delete tenant user, HTTP status: ", $httpResponse.status
[inline-code-end]

---