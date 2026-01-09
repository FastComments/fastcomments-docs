[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

このルートは `createdAt` でソートされた最大30件の `Subscription` オブジェクトを返します（新しい順）。

`userId` でフィルターできます。SSO を使用している場合、ユーザーIDは `<tenant id>:<user id>` の形式です。

[inline-code-attrs-start title = 'ユーザーのサブスクリプション cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'サブスクリプションのリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** レコードをスキップしてページングします。 **/
    skip?: number
    /** ユーザーでフィルターします。 **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'サブスクリプションのレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失敗時に含まれます。 **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---