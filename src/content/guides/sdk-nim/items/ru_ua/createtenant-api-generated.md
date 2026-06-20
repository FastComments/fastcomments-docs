## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantBody | CreateTenantBody | Нет |  |

## Ответ

Возвращает: [`Option[CreateTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenant(
  tenantId = "my-tenant-123",
  createTenantBody = CreateTenantBody(
    name = "My Tenant 123",
    domain = "news.example.com",
    allowAnonymous = false,
    allowedOrigins = @["https://news.example.com", "https://api.news.example.com"],
    description = "Comments for News Example"
  )
)
if response.isSome:
  let created = response.get()
  echo "Created tenant: ", created.tenantId
else:
  echo "Failed to create tenant, status: ", httpResponse.status
[inline-code-end]

---