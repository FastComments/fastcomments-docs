[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

此路由提供依 id 刪除 `TenantUser` 的功能。

可透過 `deleteComments` 查詢參數刪除使用者的評論。請注意若此值為 true：

1. 該使用者的所有評論將會即時刪除。
2. 所有 __child__（現為孤兒）的評論將依各評論所屬頁面的設定被刪除或匿名化。例如若執行緒刪除模式為 "anonymize"，則回覆會保留，使用者的評論會被匿名化。此行為僅適用於 `commentDeleteMode` 為 `Remove`（預設值）時。
3. `creditsCost` 會變為 `2`。

### 匿名化的評論

您可以保留使用者的評論，但透過設定 `commentDeleteMode=1` 來將其匿名化。

若使用者的評論被匿名化，則下列欄位會被設為 null：

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` 及 `isDeletedUser` 會被設為 `true`。

在渲染時，評論元件會使用 `DELETED_USER_PLACEHOLDER`（預設："[deleted]"）作為使用者名稱，並使用 `DELETED_CONTENT_PLACEHOLDER` 作為評論內容。這些可透過 Widget 自訂 UI 來自訂化。

### 範例

[inline-code-attrs-start title = 'TenantUser 刪除 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 刪除請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // 預設
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** 您可以將此設為 true 來同時刪除使用者的評論。這會使點數成本加倍。 **/
    deleteComments?: 'true' | 'false'
    /** 您可以依需求設定以決定如何處理使用者的評論。 **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 刪除回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** 於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** 於失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---