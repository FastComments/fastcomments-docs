---
[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

この API エンドポイントは、特定のユーザーのコメントのフラグ解除を行う機能を提供します。

Notes:

- この呼び出しは常にユーザーのコンテキストで行う必要があります。ユーザーは FastComments.com のユーザー、SSO ユーザー、またはテナントユーザーである可能性があります。
- コメントが自動的に承認取り消し（非表示）された後、そのコメントを再承認できるのは管理者またはモデレーターのみです。フラグ解除はコメントを再承認しません。

[inline-code-attrs-start title = 'コメントのフラグ解除 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

匿名でのフラグ付けの場合、`anonUserId` を指定する必要があります。これは匿名セッションを表す ID、またはランダムな UUID にすることができます。

[inline-code-attrs-start title = '匿名コメントのフラグ解除 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'コメントのフラグ解除リクエストの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'コメントのフラグ解除レスポンスの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---