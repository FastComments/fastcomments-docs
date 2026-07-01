## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| addDomainConfigParams | AddDomainConfigParams | No |  |

## Одговор

Враћа: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## Пример

[inline-code-attrs-start title = 'addDomainConfig Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.addDomainConfig(
  tenantId = "my-tenant-123",
  addDomainConfigParams = default(AddDomainConfigParams)
)

if respOpt.isSome:
  let cfg = respOpt.get()
[inline-code-end]