[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

此路由可透過使用者的 id 刪除單一 SSO 使用者。

請注意，若再次以該使用者的 payload 載入留言小工具，系統會無縫地重新建立該使用者。

可透過查詢參數 `deleteComments` 刪除此使用者的留言。若此參數為 true，請注意：

1. 該使用者的所有留言將會即時刪除。
2. 所有的 __子留言__（現為孤兒留言）將依據各留言所屬頁面的設定，被刪除或匿名化。例如，若討論串刪除模式為 "anonymize"，則回覆會保留，而該使用者的留言會被匿名化。此行為僅適用於 `commentDeleteMode` 為 `Remove`（預設值）時。
3. `creditsCost` 會變為 `2`。

### 匿名化留言

您可以保留該使用者的留言，但透過將 `commentDeleteMode=1` 將其匿名化。

若使用者的留言被匿名化，則下列欄位會被設為 null：

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` 與 `isDeletedUser` 將被設為 `true`。

在渲染時，留言小工具會在使用者名稱處使用 `DELETED_USER_PLACEHOLDER`（預設："[deleted]"），並在留言內容處使用 `DELETED_CONTENT_PLACEHOLDER`。可透過 Widget Customization UI 自訂這些內容。

### 範例

[inline-code-attrs-start title = 'SSOUser 移除 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 移除請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // 預設值
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** 您可以將此設為 true 以同時刪除此使用者的留言。這會使點數成本加倍。 **/
    deleteComments?: 'true' | 'false'
    /** 您可以依需求設置此值以決定如何處理該使用者的留言。 **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 移除回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** 失敗時會包含。 **/
    reason?: string
    user?: SSOUser; // 成功時會回傳被移除的使用者。
}
[inline-code-end]

---