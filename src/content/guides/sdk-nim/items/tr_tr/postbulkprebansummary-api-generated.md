## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | Hayır |  |
| includeByUserIdAndEmail | bool | Hayır |  |
| includeByIP | bool | Hayır |  |
| includeByEmailDomain | bool | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[BulkPreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_pre_ban_summary.nim)

## Örnek

[inline-code-attrs-start title = 'postBulkPreBanSummary Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---