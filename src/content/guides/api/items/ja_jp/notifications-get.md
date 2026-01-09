[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

このルートは `createdAt` でソートされた最大30件の `Notification` オブジェクトを返します。新しいものが先に来ます。

`userId` でフィルタできます。SSO を使用する場合、ユーザーIDは `<tenant id>:<user id>` の形式です。

[inline-code-attrs-start title = 'ユーザーの未読通知の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '通知リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** レコードをスキップしてページングします。 **/
    skip?: number
    /** ユーザーでフィルタします。 **/
    userId?: string
    /** urlIdでフィルタします。 **/
    urlId?: string
    /** 元のコメントでフィルタします。 **/
    fromCommentId?: string
    /** 既読/未読でフィルタします。 **/
    viewed?: 'true' | 'false'
    /** タイプでフィルタします。 **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '通知レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失敗時に含まれます。 **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]