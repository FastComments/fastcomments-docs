---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | 아니오 |  |
| includeByUserIdAndEmail | bool | 아니오 |  |
| includeByIP | bool | 아니오 |  |
| includeByEmailDomain | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[BulkPreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_pre_ban_summary.nim)

## 예제

[inline-code-attrs-start title = 'postBulkPreBanSummary 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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