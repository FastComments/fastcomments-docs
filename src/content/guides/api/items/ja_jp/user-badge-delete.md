---
このエンドポイントでは、ユーザーバッジの割り当てを削除できます。

リクエストの例:

[inline-code-attrs-start title = 'DELETE リクエストの例'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

レスポンスの例:

[inline-code-attrs-start title = 'レスポンス'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

考えられるエラー応答:

[inline-code-attrs-start title = 'エラー: テナントIDが欠落しています'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'エラー: IDが欠落しています'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'エラー: 見つかりません'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]
---