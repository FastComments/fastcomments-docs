## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | No |  |
| includeByUserIdAndEmail | bool | No |  |
| includeByIP | bool | No |  |
| includeByEmailDomain | bool | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[BulkPreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_pre_ban_summary.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postBulkPreBanSummary'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let bulkParams = BulkPreBanParams(
  tenantId = "my-tenant-123",
  userIds = @["user-456", "user-789"],
  emails = @["spammer@example.com", "bot@malicious.org"],
  ipAddresses = @["203.0.113.5", "198.51.100.42"],
  emailDomains = @["malicious.org"]
)

let (response, httpResponse) = client.postBulkPreBanSummary(
  bulkPreBanParams = bulkParams,
  includeByUserIdAndEmail = true,
  includeByIP = true,
  includeByEmailDomain = false,
  sso = "sso-token-abc123"
)

if response.isSome:
  let summary = response.get()
  echo "Pre-ban summary:", summary
[inline-code-end]