## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| domain | string | Ні |  |

## Відповідь

Повертає: [`Option[DeleteDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let result = response.get()
  echo "Deleted domain config result: ", result
else:
  echo "No response body, HTTP status: ", $httpResponse.status
[inline-code-end]