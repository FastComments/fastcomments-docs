[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

この API エンドポイントは、特定のユーザーのためにコメントにフラグを付ける機能を提供します。

注意:

- この呼び出しは常にユーザーのコンテキストで行う必要があります。ユーザーは FastComments.com のユーザー、SSO ユーザー、またはテナントユーザーであり得ます。
- フラグで非表示にする閾値が設定されている場合、定義された回数フラグが付けられるとコメントは自動的にライブで非表示になります。
- 自動的に非承認（非表示）された後、コメントを再承認できるのは管理者またはモデレーターのみです。フラグを解除してもコメントは再承認されません。

[inline-code-attrs-start title = 'コメントのフラグ付け cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

匿名でフラグを付ける場合、`anonUserId` を指定する必要があります。これは匿名セッションを表す ID やランダムな UUID にすることができます。これにより、ユーザーがログインしていない場合でもコメントのフラグ付け/フラグ解除をサポートできます。つまり、同じ `anonUserId` でコメントを取得すると、コメントはフラグ済みとしてマークされます。

[inline-code-attrs-start title = '匿名コメントのフラグ付け cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'コメントのフラグ リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'コメントのフラグ レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** コメントがフラグの付け過ぎにより非承認（非表示）になったかどうか？ **/
    wasUnapproved?: boolean;
}
[inline-code-end]