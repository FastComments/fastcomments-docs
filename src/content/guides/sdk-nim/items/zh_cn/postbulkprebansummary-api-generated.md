## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| bulkPreBanParams | BulkPreBanParams | 否 |  |
| includeByUserIdAndEmail | bool | 否 |  |
| includeByIP | bool | 否 |  |
| includeByEmailDomain | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[BulkPreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_pre_ban_summary.nim)

## 示例

[inline-code-attrs-start title = 'postBulkPreBanSummary 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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