## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domainToUpdate | string | Не |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Не |  |

## Отговор

Връща: [`Option[PutDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_domain_config_response.nim)

## Пример

[inline-code-attrs-start title = 'putDomainConfig пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "example.com",
  updateDomainConfigParams = UpdateDomainConfigParams()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]