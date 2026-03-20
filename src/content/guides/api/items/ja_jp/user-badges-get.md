このエンドポイントは、さまざまな条件に基づいてユーザーのバッジを取得するためのものです。

リクエスト例:

[inline-code-attrs-start title = 'ユーザーバッジ一覧 - GET 例'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

結果を絞り込むために、さまざまなクエリパラメータを追加できます:

- `userId` - 特定のユーザーのバッジを取得
- `badgeId` - 特定のバッジのインスタンスを取得
- `type` - バッジの種類で絞り込む（0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, など。完全な一覧は UserBadge 構造を参照してください）
- `displayedOnComments` - バッジがコメント上に表示されるかどうかで絞り込む（true/false）
- `limit` - 返されるバッジの最大数（デフォルト 30、最大 200）
- `skip` - スキップするバッジの数（ページネーション用）

レスポンス例:

[inline-code-attrs-start title = 'レスポンス'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
      "id": "badge123",
      "userId": "user456",
      "badgeId": "badgeDef789",
      "fromTenantId": "tenant001",
      "createdAt": 1650532511000,
      "receivedAt": 1650532511000,
      "type": 14,
      "name": "Special Contributor",
      "description": "Awarded to special contributors to our community",
      "displayLabel": "Special",
      "backgroundColor": "#4a5568",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 1
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
    }
  ]
}
[inline-code-end]

考えられるエラー応答:

[inline-code-attrs-start title = 'エラー: テナントIDがありません'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'エラー: 無効な limit'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]