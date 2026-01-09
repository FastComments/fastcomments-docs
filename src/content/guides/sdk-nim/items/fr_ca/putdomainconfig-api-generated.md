## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| domainToUpdate | string | Non |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Non |  |

## Réponse

Renvoie : [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple pour putDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdateDomainConfigParams(
  allowAnonymous = false,
  moderationEnabled = true,
  allowedOrigins = @["https://news.example.com"],
  maxCommentLength = 2000
)

let (response, httpResponse) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news/example-article",
  updateDomainConfigParams = updateParams
)

if response.isSome:
  let domainCfg = response.get()
  discard domainCfg
[inline-code-end]

---