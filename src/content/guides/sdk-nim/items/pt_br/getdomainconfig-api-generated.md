## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| domain | string | Não |  |

## Resposta

Retorna: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if response.isSome:
  let domainConfig = response.get()
  echo "Loaded domain config for tenant my-tenant-123:", $domainConfig
else:
  echo "No domain config; HTTP status:", $httpResponse.status
[inline-code-end]