[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

このAPIはページネーションを使用しており、`skip` クエリパラメータで提供されます。モデレーターは `100` 件ごとのページで返され、`createdAt` と `id` の順で並びます。

コストは返されるモデレーターの数に基づき、`1 credit per 10` モデレーターごとに課金されます。

[inline-code-attrs-start title = 'モデレーターの cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'モデレーターのリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** ページネーションのためにスキップするモデレーターの数。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'モデレーターのレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---