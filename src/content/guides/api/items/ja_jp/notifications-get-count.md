[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

このルートは、`count` パラメータの下に通知の数を含むオブジェクトを返します。

これは `/notification-count/` より遅く、クレジットの消費は2倍ですが、より多くの次元でフィルタリングできます。

`/notifications` エンドポイントと同じパラメータ（例：`userId`）でフィルタリングできます。SSO を使用している場合、ユーザーID は `<tenant id>:<user id>` の形式になります。

[inline-code-attrs-start title = 'ユーザーの未読通知数（cURL例）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '特定ページのユーザーの未読通知数（cURL例）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '通知数リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
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

[inline-code-attrs-start title = '通知数レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失敗時に含まれます。 **/
    reason?: string
    count?: number
}
[inline-code-end]