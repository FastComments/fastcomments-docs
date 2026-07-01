## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Ответ

Возвращает: [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## Пример

[inline-code-attrs-start title = 'getDomainConfigs Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (domainConfigsOpt, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if domainConfigsOpt.isSome:
  let domainConfigs = domainConfigsOpt.get()
  echo domainConfigs
[inline-code-end]