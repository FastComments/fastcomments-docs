[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

このAPIはページネーションを使用しており、`skip` クエリパラメータで指定します。TenantUsers は `100` 件ずつのページで返され、`signUpDate`、`username`、`id` の順に並べられます。

コストは返されるテナントユーザーの数に基づき、返されるテナントユーザー10件ごとに `1 credit per 10` の費用がかかります。

[inline-code-attrs-start title = 'TenantUser の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** ページネーションでスキップするテナントユーザーの数。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---