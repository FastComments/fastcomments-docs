[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントはコメントを削除するための機能を提供します。

Notes:

- このAPIは、必要に応じてコメントウィジェットを「ライブ」で更新できます（これにより `creditsCost` が `1` から `2` に増加します）。
- このAPIはすべての子コメントを削除します。

[inline-code-attrs-start title = 'コメント DELETE cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** 更新を行っているユーザー。必要であれば、コメントを削除できるかどうかを確認するために使用できます。 **/
    contextUserId?: string
	/** 同じ urlId のコメントウィジェットのインスタンスを閲覧しているユーザーに対してコメントを「ライブ」で削除するかどうか。注: クレジットコストが 1 から 2 に倍増します。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'コメント DELETE レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---