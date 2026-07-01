## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| domain | string | Non |  |

## Réponse

Renvoie : [`Option[DeleteDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config_response.nim)

## Exemple

[inline-code-attrs-start title = 'deleteDomainConfig Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---