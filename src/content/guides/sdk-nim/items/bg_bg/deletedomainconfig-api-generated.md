## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domain | string | Не |  |

## Отговор

Връща: [`Option[DeleteDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config_response.nim)

## Пример

[inline-code-attrs-start title = 'deleteDomainConfig Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let deleted = response.get()
  echo "DeleteDomainConfig succeeded for tenant ", "my-tenant-123"
else:
  echo "DeleteDomainConfig failed. HTTP status: ", $httpResponse.status
[inline-code-end]