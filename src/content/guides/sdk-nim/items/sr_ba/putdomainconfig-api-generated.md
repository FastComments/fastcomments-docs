## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| domainToUpdate | string | Ne |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Ne |  |

## Odgovor

Vraća: [`Option[PutDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_domain_config_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer putDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "blog.example.com",
  updateDomainConfigParams = UpdateDomainConfigParams(
    allowAnonymous = false,
    moderationEnabled = true,
    maxCommentLength = 800,
    allowedOrigins = @["https://blog.example.com", "https://cdn.blog.example.com"],
    enableThreadedComments = true
  )
)

if response.isSome:
  let cfg = response.get()
  echo cfg
else:
  echo "Failed to update domain config, HTTP status: ", httpResponse.status
[inline-code-end]

---