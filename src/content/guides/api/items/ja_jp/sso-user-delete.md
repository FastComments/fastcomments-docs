[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

このルートは、IDによって単一のSSOユーザーを削除します。

このユーザーのペイロードでコメントウィジェットを再度読み込むと、そのユーザーはシームレスに再作成されることに注意してください。

ユーザーのコメントは `deleteComments` クエリパラメータで削除できます。これが true の場合、次のことに注意してください：

1. ユーザーのすべてのコメントが即時に削除されます。
2. すべての __child__（現在孤立した）コメントは、各コメントに紐づくページの設定に基づいて削除または匿名化されます。たとえば、スレッド削除モードが "anonymize" の場合は返信は残り、ユーザーのコメントは匿名化されます。これは `commentDeleteMode` が `Remove`（デフォルト値）の場合にのみ適用されます。
3. `creditsCost` は `2` になります。

### 匿名化されたコメント

ユーザーのコメントを保持したまま匿名化するには、`commentDeleteMode=1` に設定します。

ユーザーのコメントが匿名化されると、次の値が null に設定されます：

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` と `isDeletedUser` は `true` に設定されます。

レンダリング時、コメントウィジェットはユーザー名に `DELETED_USER_PLACEHOLDER`（デフォルト: "[deleted]"）を、コメントには `DELETED_CONTENT_PLACEHOLDER` を使用します。これらはウィジェットカスタマイズUIから変更できます。

### 例

[inline-code-attrs-start title = 'SSOUser 削除の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 削除リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // デフォルト
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** ユーザーのコメントも削除するにはこれを true に設定できます。これによりクレジットコストが2倍になります。 **/
    deleteComments?: 'true' | 'false'
    /** ユーザーのコメントの扱い方法を決定するために任意に設定できます。 **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 削除レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** 失敗時に含まれます。 **/
    reason?: string
    user?: SSOUser; // 成功時に削除されたユーザーを返します。
}
[inline-code-end]