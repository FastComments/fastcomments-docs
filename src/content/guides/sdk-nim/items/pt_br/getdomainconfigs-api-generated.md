## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |

## Resposta

Retorna: [`Option[GetDomainConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_configs_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getDomainConfigs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (domainConfigsOpt, httpResponse) = client.getDomainConfigs(tenantId = "my-tenant-123")
if domainConfigsOpt.isSome:
  let domainConfigs = domainConfigsOpt.get()
  echo domainConfigs
[inline-code-end]