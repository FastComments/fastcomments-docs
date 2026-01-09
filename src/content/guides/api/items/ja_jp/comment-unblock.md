[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントは、指定されたコメントを書いたユーザーのブロック解除を行う機能を提供します。FastComments.com ユーザー、SSO ユーザー、およびテナントユーザーによって書かれたコメントからのブロック解除に対応しています。

この操作実行後にクライアント上で他に表示される可能性のあるコメントをブロック/ブロック解除すべきかを確認するための `commentIdsToCheck` ボディパラメータをサポートします。

注意:

- この呼び出しは常にユーザーのコンテキストで行う必要があります。ユーザーは FastComments.com ユーザー、SSO ユーザー、またはテナントユーザーである可能性があります。
- リクエスト内の `userId` はブロック解除を行うユーザーです。例えば：`User A` が `User B` のブロックを解除したい場合、`userId=User A` と `User B` が書いたコメントのIDを渡します。
- ユーザーIDもメールアドレスもない完全に匿名のコメントはブロックできず、エラーが返されます。

[inline-code-attrs-start title = 'コメントのブロック解除 cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '匿名コメントのブロック解除 cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'コメントのブロック解除リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'コメントのブロック解除レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** commentIdsToCheck が定義されている場合、このマップ内で true のエントリは引き続きブロックされています。false の場合は、ユーザーが再読み込みしなくても済むようにコメントの非表示を解除することを検討してください。 **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---