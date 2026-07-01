## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| domainToUpdate | string | Não |  |
| patchDomainConfigParams | PatchDomainConfigParams | Não |  |

## Resposta

Retorna: [`Option[PatchDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_domain_config_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo patchDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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