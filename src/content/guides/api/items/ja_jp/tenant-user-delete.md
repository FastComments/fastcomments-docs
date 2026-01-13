[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

このルートは id によって `TenantUser` を削除します。

ユーザーのコメントを削除することは、`deleteComments` クエリパラメータで可能です。これが true の場合は注意してください:

1. そのユーザーのコメントはすべてライブで削除されます。
2. すべての __child__（現在孤立した）コメントは、各コメントに関連付けられたページの設定に基づいて削除または匿名化されます。例えば、スレッド削除モードが "anonymize" の場合、返信は残り、ユーザーのコメントは匿名化されます。これは `commentDeleteMode` が `Remove`（デフォルト値）の場合にのみ適用されます。
3. `creditsCost` は `2` になります。

### 匿名化されたコメント

ユーザーのコメントを保持したまま `commentDeleteMode=1` を設定して単に匿名化することができます。

ユーザーのコメントが匿名化されると、次の値が null に設定されます:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` と `isDeletedUser` は `true` に設定されます。

レンダリング時、コメントウィジェットはユーザー名に `DELETED_USER_PLACEHOLDER`（デフォルト: "[deleted]"）、コメントには `DELETED_CONTENT_PLACEHOLDER` を使用します。これらはウィジェットカスタマイズ UI でカスタマイズ可能です。

### 例

[inline-code-attrs-start title = 'TenantUser 削除 cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 削除リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // デフォルト
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** ユーザーのコメントも削除するには、これを true に設定できます。これによりクレジットコストが2倍になります。 **/
    deleteComments?: 'true' | 'false'
    /** ユーザーのコメントの取り扱い方法を決定するために、必要に応じてこれを設定できます。 **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 削除レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---