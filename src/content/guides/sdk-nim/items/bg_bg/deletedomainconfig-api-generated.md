## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domain | string | Не |  |

## Отговор

Връща: [`Option[DeleteDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let result = response.get()
  echo "Deleted domain config result: ", result
else:
  echo "No response body, HTTP status: ", $httpResponse.status
[inline-code-end]

---