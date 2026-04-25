---
[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントはコメントを削除する機能を提供します。

Notes:

- 必要に応じて、このAPIはコメントウィジェットをライブで更新できます（これにより `creditsCost` が `1` から `2` に増加します）。
- このAPIはすべての子コメントを削除します。
- 対象のコメントがロックされている場合（`isLocked: true`）、リクエストは `code: 'locked'` で拒否されます。先にコメントのロックを解除してから削除してください。

[inline-code-attrs-start title = 'コメント DELETE cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'コメント DELETE リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** 更新を行うユーザー。必要に応じて、そのユーザーがコメントを削除できるかどうか確認するために使用できます。  **/
    contextUserId?: string
	/** 同じ urlId を持つコメントウィジェットのインスタンスを閲覧しているユーザーに対して、コメントを「ライブ」で削除するかどうか。NOTE: クレジットコストが `1` から `2` に倍増します。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'コメント DELETE レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---