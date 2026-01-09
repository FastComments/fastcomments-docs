---
此端點允許您根據各種條件取得使用者徽章進度紀錄。

範例請求：

[inline-code-attrs-start title = 'GET 請求範例'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

您可以加入各種查詢參數來篩選結果：

- `userId` - 取得特定使用者的進度紀錄
- `limit` - 要返回的最大紀錄數（預設 30，最大 200）
- `skip` - 要跳過的紀錄數（用於分頁）

範例回應：

[inline-code-attrs-start title = '回應'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgresses": [
    {
      "id": "progress123",
      "tenantId": "tenant001",
      "userId": "user456",
      "firstCommentId": "comment789",
      "firstCommentDate": 1650532511000,
      "autoTrustFactor": 0.75,
      "progress": {
        "0": 42,
        "1": 120,
        "2": 15,
        "3": 3,
        "5": 5,
        "6": 1800000,
        "8": 0,
        "7": 0
      }
    },
    {
      "id": "progress124",
      "tenantId": "tenant001",
      "userId": "user789",
      "firstCommentId": "comment790",
      "firstCommentDate": 1650532598000,
      "autoTrustFactor": 0.5,
      "progress": {
        "0": 12,
        "1": 15,
        "2": 4
      }
    }
  ]
}
[inline-code-end]

可能的錯誤回應：

[inline-code-attrs-start title = '錯誤：缺少租戶 ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = '錯誤：無效的限制'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]
---