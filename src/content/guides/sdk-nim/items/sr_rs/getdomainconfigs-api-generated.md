## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Одговор

Враћа: [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## Пример

[inline-code-attrs-start title = 'getDomainConfigs Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if response.isSome:
  let domainConfigs = response.get()
  echo "Received domain configs for tenant my-tenant-123"
  echo domainConfigs
else:
  echo "No domain configs returned"
[inline-code-end]

---