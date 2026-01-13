このエンドポイントは、固有のIDで特定のユーザーバッジ進捗レコードを取得できます。

リクエストの例:

[inline-code-attrs-start title = 'GET リクエストの例'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/progress123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

レスポンスの例:

[inline-code-attrs-start title = 'レスポンス'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgress": {
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
  }
}
[inline-code-end]

想定されるエラー応答:

[inline-code-attrs-start title = 'エラー: テナントIDが見つかりません'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'エラー: 見つかりません'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge progress progress123 was not found."
}
[inline-code-end]