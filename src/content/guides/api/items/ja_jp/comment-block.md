[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントは、与えられたコメントを書いたユーザーをブロックする機能を提供します。FastComments.com ユーザー、SSO ユーザー、およびテナントユーザーによって書かれたコメントからのブロックをサポートします。

この操作の実行後に、クライアント上で他に表示されている可能性のあるコメントをブロック/ブロック解除すべきかを確認するための `commentIdsToCheck` ボディパラメータをサポートします。

注意事項:

- この呼び出しは常にユーザーのコンテキストで行う必要があります。ユーザーは FastComments.com ユーザー、SSO ユーザー、またはテナントユーザーであり得ます。
- リクエスト内の `userId` はブロックを実行するユーザーです。例えば: `User A` が `User B` をブロックしたい場合、`userId=User A` と `User B` が書いたコメントのIDを渡します。
- 完全に匿名のコメント（ユーザーIDもメールアドレスもない）はブロックできず、エラーが返されます。

[inline-code-attrs-start title = 'コメントブロック cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

匿名でのブロックでは、`anonUserId` を指定する必要があります。これは匿名セッションを表すID、またはランダムなUUIDでも構いません。
これにより、ユーザーがログインしていない場合でも、同じ `anonUserId` でコメントを取得することでコメントのブロックをサポートできます。

[inline-code-attrs-start title = '匿名コメントブロック cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'コメントブロック リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'コメントブロック レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** もし commentIdsToCheck が定義されている場合、このマップ内で true のエントリもブロックされます。 **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---