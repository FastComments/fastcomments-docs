[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

この API はページネーションを使用しており、`skip` クエリパラメータで指定します。TenantPackages は `100` 件ごとのページで返され、`createdAt` と `id` の順でソートされます。

コストは返される tenant packages の数に基づき、10 件ごとに `1 credit` がかかります。

[inline-code-attrs-start title = 'TenantPackage の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** ページネーションでスキップするテナントパッケージの数。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---