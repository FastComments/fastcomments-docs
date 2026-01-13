[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

このAPIは、あなたのテナントによって管理されているテナントを返します。

ページネーションは `skip` クエリパラメータで提供されます。テナントは `100` 件ずつのページで返され、`signUpDate` と `id` の順で並びます。

コストは返されるテナントの数に基づき、返される10件ごとに `1 credit per 10` がかかります。

[inline-code-attrs-start title = 'テナントの cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

`Tenant` オブジェクトに `meta` パラメータを定義して一致するテナントをクエリできます。例えば、キーが `someKey` でメタ値が `some-value` の場合、このキー/値ペアを含むJSONオブジェクトを作成し、それをURIエンコードしてクエリパラメータとして渡してフィルタリングできます:

[inline-code-attrs-start title = 'メタによるテナントクエリの cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'テナントのリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** ページネーションのためにスキップするテナントの数。 **/
    skip?: number
    /** meta パラメータでフィルタ。 **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'テナントのレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---