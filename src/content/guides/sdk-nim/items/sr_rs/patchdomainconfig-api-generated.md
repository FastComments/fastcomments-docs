## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domainToUpdate | string | Не |  |
| patchDomainConfigParams | PatchDomainConfigParams | Не |  |

## Одговор

Враћа: [`Option[PatchDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_domain_config_response.nim)

## Пример

[inline-code-attrs-start title = 'patchDomainConfig Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.patchDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news.mywebsite.com",
  patchDomainConfigParams = PatchDomainConfigParams()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]